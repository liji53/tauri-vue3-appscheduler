use super::base_app::RepoCommand;
use super::database::db_session;
use super::svn_app::SvnRepo;
use tokio_cron_scheduler::{Job, JobScheduler};

static mut SCHEDULER: Option<JobScheduler> = None;

fn get_sched() -> &'static JobScheduler {
    unsafe { SCHEDULER.as_ref().unwrap() }
}

/// 启动定时任务
pub async fn init_scheduler() -> Result<(), String> {
    // 创建scheduler
    let sched = JobScheduler::new()
        .await
        .map_err(|e| format!("JobScheduler创建失败：{e}"))?;

    unsafe {
        SCHEDULER = Some(sched);
    }

    let conn = db_session(Some("启动scheduler失败"));
    if let Err(e) = conn {
        println!("{e}");
        // 启动scheduler
        return get_sched()
            .start()
            .await
            .map_err(|e| format!("JobScheduler启动失败: {e}"));
    }
    // 查询设置了定时且状态为True的任务
    let conn = conn.unwrap();
    let sql_stmt = "SELECT id, cron, url, name FROM task WHERE status = 1 AND cron != ''";
    let stmt = conn.prepare(sql_stmt);
    if let Err(e) = stmt {
        println!("{e}");
        return get_sched()
            .start()
            .await
            .map_err(|e| format!("JobScheduler启动失败: {e}"));
    }
    let mut stmt = stmt.unwrap();
    let job_iter = stmt
        .query_map([], |row| {
            let row_tuple: (u32, String, String, String) =
                (row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?);
            Ok(row_tuple)
        })
        .map_err(|_| "数据库查询异常!")?;

    // 将任务转成Job，并添加到scheduler
    for job in job_iter.into_iter() {
        let job = job.map_err(|e| format!("数据库查询结果异常：{e}"))?;
        let cron = job.1.as_str();
        let url = job.2;
        let job_name = job.3;
        let task = Job::new(cron, move |_uuid, _l| {
            let svn_repo = SvnRepo::new(url.to_string());
            if let Err(e) = svn_repo.run_app(None, job_name.to_string(), job.0) {
                println!("{job_name}失败：{e}");
            }
        })
        .map_err(|e| format!("Job::new创建失败：{e}"))?;
        get_sched()
            .add(task)
            .await
            .map_err(|e| format!("JobScheduler添加任务失败: {e}"))?;
    }

    // 启动scheduler
    get_sched()
        .start()
        .await
        .map_err(|e| format!("JobScheduler启动失败: {e}"))
}
