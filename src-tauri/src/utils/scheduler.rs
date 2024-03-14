use std::collections::HashMap;

use super::base_app::RepoCommand;
use super::svn_app::SvnRepo;
use crate::task_manager::view::get_cron_tasks;
use tokio_cron_scheduler::{Job, JobScheduler};
use uuid::Uuid;

static mut SCHEDULER: Option<JobScheduler> = None;
static mut JOB_GUIDS: Option<HashMap<u32, Uuid>> = None;

fn get_sched() -> &'static JobScheduler {
    unsafe { SCHEDULER.as_ref().unwrap() }
}
fn insert_guid(id: u32, guid: Uuid) {
    unsafe {
        JOB_GUIDS.as_mut().unwrap().insert(id, guid);
    }
}
fn get_guid(id: u32) -> Option<Uuid> {
    unsafe {
        if let Some(guid) = JOB_GUIDS.as_ref().unwrap().get(&id) {
            Some(guid.clone())
        } else {
            None
        }
    }
}
fn delete_guid(id: u32) {
    unsafe {
        JOB_GUIDS.as_mut().unwrap().remove(&id);
    }
}

/// 启动定时任务
pub async fn init_scheduler() -> Result<(), String> {
    // 创建scheduler
    let sched = JobScheduler::new().await.expect("JobScheduler创建失败!");

    unsafe {
        SCHEDULER = Some(sched);
        JOB_GUIDS = Some(HashMap::new());
    }

    // 将符合条件的task加入到scheduler中
    let jobs = get_cron_tasks();
    if let Ok(jobs) = jobs {
        for job in jobs.into_iter() {
            let job = job.unwrap();
            if let Err(e) = add_job(job.0, job.1, job.2, job.3).await {
                print!("{e}");
            }
        }
    }

    // 启动scheduler
    get_sched()
        .start()
        .await
        .map_err(|e| format!("JobScheduler启动失败: {e}"))
}

/// 添加job到scheduler
pub async fn add_job(id: u32, cron: String, url: String, name: String) -> Result<(), String> {
    // 新建job
    let task = Job::new(cron.as_str(), move |_uuid, _l| {
        let svn_repo = SvnRepo::new(url.to_string());
        if let Err(e) = svn_repo.run_app(None, name.to_string(), id) {
            println!("{name}失败：{e}");
        }
    })
    .map_err(|e| format!("Job::new创建失败：{e}"))?;

    // 保存job_id
    insert_guid(id, task.guid());

    // 加入到scheduler
    get_sched()
        .add(task)
        .await
        .map_err(|e| format!("JobScheduler添加任务失败: {e}"))?;

    Ok(())
}

/// 更新job
pub async fn modify_job(id: u32, cron: String, url: String, name: String) -> Result<(), String> {
    remove_job(id).await?;
    if cron == "" {
        return Ok(());
    }
    add_job(id, cron, url, name).await
}

/// 从scheduler中删除job
pub async fn remove_job(id: u32) -> Result<(), String> {
    if let Some(job_id) = get_guid(id) {
        get_sched()
            .remove(&job_id)
            .await
            .map_err(|e| format!("从scheduler中删除job异常：{e}"))?
    }
    delete_guid(id);
    Ok(())
}
