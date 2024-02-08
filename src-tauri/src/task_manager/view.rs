use super::schemas::{Job, JobCreate, JobModel, JobPagination, JobUpdate};
use super::service;
use crate::utils;
use crate::utils::base_app::RepoCommand;
use tauri::Window;

// todo: next_at 的实现
#[tauri::command]
pub fn get_jobs(
    name: String,
    category: String,
    page: u32,
    items_per_page: u32,
) -> Result<JobPagination, String> {
    let error = "获取任务列表失败";
    // 查询数据库，获取所有的任务列表
    let conn = utils::db_session(Some(error))?;
    let jobs = service::get_all(&conn).map_err(|e| format!("{}, {}", error, e))?;
    let jobs: Vec<Job> = jobs
        .into_iter()
        .map(|job| Job {
            id: job.id.unwrap(),
            name: job.name,
            remark: job.remark,
            status: job.status,
            next_at: "2024-02-07 03:26:00".to_string(),
            cron: job.cron,
            app_name: job.app_name,
            category: job.category,
            url: job.url,
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
    let job: JobCreate =
        serde_json::from_str(&data).map_err(|_| format!("{}, 参数转json失败", error))?;

    // 读取远程仓库中的配置文件, 并找到对应的配置项
    let app_config = utils::cached_app_store_config().map_err(|e| format!("{}, {}", error, e))?;
    if let Some(app_store_item) = app_config
        .app_list
        .iter()
        .find(|item| item.name == job.app_name)
    {
        // db创建job
        let conn = utils::db_session(Some(error))?;
        service::create(
            &conn,
            &JobModel {
                id: None,
                name: job.name,
                remark: job.remark,
                status: true,
                cron: "".to_string(),
                app_name: job.app_name,
                category: app_store_item.category.to_string(),
                url: app_store_item.url.to_string(),
            },
        )
        .map_err(|e| format!("{}, {}", error, e))
    } else {
        Err(format!("{}, 配置文件中不存在该应用", error))
    }
}

/// 删除任务
#[tauri::command]
pub fn delete_job(id: u32) -> Result<(), String> {
    let error = "删除任务失败";
    let conn = utils::db_session(Some(error))?;

    service::delete(&conn, id).map_err(|e| format!("{}, {}", error, e))
}

/// 更新任务(包括 状态、cron)
#[tauri::command]
pub fn update_job(id: u32, data: String) -> Result<(), String> {
    let error = "更新任务失败";
    let job_in: JobUpdate =
        serde_json::from_str(&data).map_err(|_| format!("{}, 参数转json失败", error))?;
    // println!("{:?}", job_in);
    let conn = utils::db_session(Some(error))?;
    service::update(&conn, id, &job_in).map_err(|e| format!("{}, {}", error, e))
}

/// 异步手动执行任务， 执行完成之后会主动通知客户端
#[tauri::command]
pub fn run_job(window: Window, id: u32) -> Result<(), String> {
    let error = "执行任务失败";
    let conn = utils::db_session(Some(error))?;
    let job = service::get_by_id(&conn, id)?;
    if job.is_none() {
        return Err(format!("{}, 该任务不存在", error));
    }

    // 异步执行任务
    let job = job.unwrap();
    let svn_repo = utils::svn_app::SvnRepo::new(job.url);
    svn_repo
        .run_app(window, job.name, job.id.unwrap())
        .map_err(|e| format!("{}, {}", error, e))
}

// #[tauri::command]
// pub fn getconfig_app(repo_url: String, app_form: String, task_id: u32) -> Result<String, String> {
//     let svn_repo: SvnRepo = SvnRepo::new(repo_url);
//     svn_repo.getconfig_app(app_form, task_id)
// }

// #[tauri::command]
// pub fn setconfig_app(repo_url: String, config: String, task_id: u32) -> Result<(), String> {
//     let svn_repo: SvnRepo = SvnRepo::new(repo_url);
//     svn_repo.setconfig_app(config, task_id)
// }
