use super::schemas::{Job, JobCreate, JobModel, JobPagination};
use super::service;
use crate::utils;
use rusqlite::{Connection, Result};
// use tauri::Window;

// todo: 查询优化, 过滤
#[tauri::command]
pub fn get_jobs(
    name: String,
    category: String,
    page: u32,
    itemsPerPage: u32,
) -> Result<JobPagination, String> {
    let error = "获取任务列表失败";
    // 查询数据库，获取任务列表
    let conn = Connection::open(utils::program_db_path())
        .map_err(|_| format!("{}, 数据库链接异常", error))?;
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
    Ok(JobPagination {
        total: jobs.len() as u32,
        data: jobs,
    })
}

#[tauri::command]
pub fn create_job(data: String) -> Result<(), String> {
    let error = "创建任务失败";
    let job: JobCreate =
        serde_json::from_str(&data).map_err(|_| format!("{}, 内部异常：参数转json失败", error))?;

    // 读取远程仓库中的配置文件, 并找到对应的配置项
    let app_config = utils::cached_app_store_config().map_err(|e| format!("{}, {}", error, e))?;
    if let Some(app_store_item) = app_config
        .app_list
        .iter()
        .find(|item| item.name == job.app_name)
    {
        // db创建job
        let conn = Connection::open(utils::program_db_path())
            .map_err(|_| format!("{}, 数据库链接异常", error))?;
        service::create_job(
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

// #[tauri::command]
// pub fn run_app(
//     window: Window,
//     repo_url: String,
//     task_name: String,
//     task_id: u32,
// ) -> Result<(), String> {
//     let svn_repo: SvnRepo = SvnRepo::new(repo_url);
//     svn_repo.run_app(window, task_name, task_id)
// }

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
