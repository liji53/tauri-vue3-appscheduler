use super::schemas::{Job, JobCreate, JobLog, JobModel, JobPagination, JobResult, JobUpdate};
use super::service;
use crate::utils;
use crate::utils::{base_app::RepoCommand, database::db_session, scheduler, svn_app::SvnRepo};
use chrono::Local;
use serde_json::Value;
use std::str::FromStr;
use std::{fs, path::Path};
use tauri::Window;

/// 获取任务列表
/// todo: 卸载应用之后，任务没有删除
#[tauri::command]
pub fn get_jobs(
    name: String,
    category: String,
    page: u32,
    items_per_page: u32,
) -> Result<JobPagination, String> {
    let error = "获取任务列表失败";
    // 查询数据库，获取所有的任务列表
    let conn = db_session(Some(error))?;
    let jobs = service::get_all(&conn).map_err(|e| format!("{}, {}", error, e))?;
    let jobs: Vec<Job> = jobs
        .into_iter()
        .map(|job| {
            let mut next_at = None;
            // cron 库不支持5位的cron表达式
            if let Ok(schedule) = cron::Schedule::from_str(&job.cron) {
                next_at = Some(
                    schedule
                        .upcoming(Local)
                        .next()
                        .unwrap()
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string(),
                );
            }
            Job {
                id: job.id.unwrap(),
                name: job.name,
                remark: job.remark,
                status: job.status,
                next_at,
                cron: job.cron,
                app_name: job.app_name,
                category: job.category,
                url: job.url,
                pre_success: job.pre_success,
            }
        })
        .collect();

    // 根据条件，过滤
    let jobs: Vec<Job> = jobs
        .into_iter()
        .filter(|item| {
            (name.is_empty() || item.name.contains(&name))
                && (category.is_empty() || item.category == category)
        })
        .collect();
    // 分页
    let jobs = utils::paginate(jobs, page, items_per_page);
    Ok(JobPagination {
        total: jobs.len() as u32,
        data: jobs,
    })
}

/// 新建任务
#[tauri::command]
pub fn create_job(data: String) -> Result<(), String> {
    let error = "创建任务失败";
    let job_in: JobCreate =
        serde_json::from_str(&data).map_err(|_| format!("{}, 客户端参数异常", error))?;

    // 读取远程仓库中的配置文件, 并找到对应的配置项
    let app_config = utils::get_app_store_config().map_err(|e| format!("{}, {}", error, e))?;
    if let Some(app_store_item) = app_config
        .app_list
        .iter()
        .find(|item| item.name == job_in.app_name)
    {
        // db创建job
        let conn = db_session(Some(error))?;
        let job = service::get_by_name(&conn, &job_in.name).map_err(|e| format!("{error}, {e}"))?;
        match job {
            None => service::create(
                &conn,
                &JobModel {
                    id: None,
                    name: job_in.name,
                    remark: job_in.remark,
                    status: true,
                    cron: "".to_string(),
                    app_name: job_in.app_name,
                    category: app_store_item.category.to_string(),
                    url: app_store_item.url.to_string(),
                    pre_success: None,
                },
            )
            .map_err(|e| format!("{}, {}", error, e)),
            Some(_) => Err(format!("{error}, 存在同名的任务")),
        }
    } else {
        Err(format!("{}, 配置文件中不存在该应用", error))
    }
}

/// 删除任务
#[tauri::command]
pub async fn delete_job(id: u32) -> Result<(), String> {
    let error = "删除任务失败";
    let conn = db_session(Some(error))?;

    service::delete(&conn, id).map_err(|e| format!("{}, {}", error, e))?;
    scheduler::remove_job(id).await
}

/// 更新任务(包括 状态、cron)
#[tauri::command]
pub async fn update_job(id: u32, data: String) -> Result<(), String> {
    let error = "更新任务失败";
    let job_in: JobUpdate =
        serde_json::from_str(&data).map_err(|_| format!("{}, 客户端参数异常", error))?;
    // println!("{:?}", job_in);
    let conn = db_session(Some(error))?;
    service::update(&conn, id, &job_in).map_err(|e| format!("{}, {}", error, e))?;
    if job_in.cron.is_some() {
        let job = service::get_by_id(&conn, id).unwrap().unwrap();
        if job.status {
            return scheduler::modify_job(id, job.cron.clone(), job.url.clone(), job.name.clone())
                .await;
        }
    } else if job_in.status.is_some() {
        let job = service::get_by_id(&conn, id).unwrap().unwrap();
        if job.status {
            return scheduler::add_job(id, job.cron.clone(), job.url.clone(), job.name.clone())
                .await;
        } else {
            return scheduler::remove_job(id).await;
        }
    }
    Ok(())
}

/// 异步手动执行任务， 执行完成之后会主动通知客户端
#[tauri::command]
pub fn run_job(window: Window, id: u32) -> Result<(), String> {
    let error = "执行任务失败";
    let conn = db_session(Some(error))?;
    let job = service::get_by_id(&conn, id)?;
    if job.is_none() {
        return Err(format!("{}, 该任务不存在", error));
    }

    // 异步执行任务
    let job = job.unwrap();
    let svn_repo = SvnRepo::new(job.url);
    svn_repo
        .run_app(Some(window), job.name, job.id.unwrap())
        .map_err(|e| format!("{}, {}", error, e))
}

/// 获取任务执行的日志
#[tauri::command]
pub fn get_job_log(id: u32) -> Result<JobLog, String> {
    let error = "获取日志失败";
    let conn = db_session(Some(error))?;
    let job = service::get_by_id(&conn, id)?;
    if job.is_none() {
        return Err(format!("{}, 该任务不存在", error));
    }

    // 读取日志内容
    let job = job.unwrap();
    let svn_repo = SvnRepo::new(job.url);
    let log_content = svn_repo
        .cat(&utils::task_log_file(id))
        .map_err(|e| format!("{error}, {e}"))?;

    // 获取日志修改时间
    let log_path = Path::new(&svn_repo.local_path()).join(utils::task_log_file(id));
    let file_metadata = fs::metadata(log_path).map_err(|e| format!("{error}, {e}"))?;
    let modified_time = file_metadata
        .modified()
        .map_err(|e| format!("{error}, {e}"))?;
    let modified_time: chrono::DateTime<chrono::Local> = modified_time.into();

    Ok(JobLog {
        created_at: format!("{}", modified_time.format("%Y-%m-%d %H:%M:%S")),
        content: log_content,
    })
}

/// 获取任务的配置
#[tauri::command]
pub fn get_job_config(id: u32) -> Result<String, String> {
    // 从DB中找到Job
    let error = "获取任务配置失败";
    let conn = db_session(Some(error))?;
    let job = service::get_by_id(&conn, id)?;
    if job.is_none() {
        return Err(format!("{}, 该任务不存在", error));
    }
    // 从项目配置中找到对应app
    let job = job.unwrap();
    let app_config = utils::get_app_store_config().map_err(|e| format!("{error}, {e}"))?;
    let app_config_item = app_config
        .app_list
        .into_iter()
        .find(|item| item.name == job.app_name);
    if app_config_item.is_none() {
        return Err(format!("{error}, 项目配置中不存在该应用!"));
    }

    // 获取任务的配置
    let svn_repo = SvnRepo::new(job.url);
    svn_repo
        .get_config(app_config_item.unwrap(), id)
        .map_err(|e| format!("{error}, {e}"))
}

/// 设置任务的配置
#[tauri::command]
pub fn set_job_config(id: u32, data: String) -> Result<(), String> {
    let error = "保存配置失败";
    let job_config_in: Value =
        serde_json::from_str(&data).map_err(|_| format!("{}, 客户端参数异常", error))?;

    // 从DB中找到Job
    let conn = db_session(Some(error))?;
    let job = service::get_by_id(&conn, id)?;
    if job.is_none() {
        return Err(format!("{}, 该任务不存在", error));
    }

    let svn_repo: SvnRepo = SvnRepo::new(job.unwrap().url);
    svn_repo
        .set_config(job_config_in, id)
        .map_err(|e| format!("{error}, {e}"))
}

/// 获取任务的执行结果
#[tauri::command]
pub fn get_job_result(id: u32) -> Result<JobResult, String> {
    let error = "获取执行结果失败";
    let conn = db_session(Some(error))?;
    let job = service::get_by_id(&conn, id)?;
    if job.is_none() {
        return Err(format!("{}, 该任务不存在", error));
    }
    let job = job.unwrap();
    let svn_repo = SvnRepo::new(job.url);
    let html_path = format!(
        "{}/{}.html",
        svn_repo.local_path().clone().replace('\\', "/"),
        id
    );

    Ok(JobResult {
        created_at: "".to_string(),
        html_path,
    })
}

/// 执行任务之后的回调 ，用于保存任务的执行结果
pub fn run_after(id: u32, success: bool) -> Result<(), String> {
    let conn = db_session(Some(""))?;
    service::update(
        &conn,
        id,
        &JobUpdate {
            name: None,
            remark: None,
            app_name: None,
            status: None,
            cron: None,
            pre_success: Some(success),
        },
    )
}

/// 查询设置了定时且状态为True的任务
pub fn get_cron_tasks(
) -> Result<Vec<Result<(u32, String, String, String), rusqlite::Error>>, String> {
    let conn = db_session(Some("init scheduler失败"))?;

    let sql_stmt = "SELECT id, cron, url, name FROM task WHERE status = 1 AND cron != ''";
    let mut stmt = conn
        .prepare(sql_stmt)
        .map_err(|e| format!("数据库查询失败: {e}"))?;

    let job_iter = stmt
        .query_map([], |row| {
            let row_tuple: (u32, String, String, String) =
                (row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?);
            Ok(row_tuple)
        })
        .map_err(|e| format!("数据库查询失败: {e}"))?;

    Ok(job_iter.into_iter().collect::<Vec<_>>())
}
