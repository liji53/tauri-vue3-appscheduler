use super::schemas::JobModel;
use rusqlite::{Connection, Result};

pub fn get_all(conn: &Connection) -> Result<Vec<JobModel>, String> {
    let mut stmt = conn
        .prepare("SELECT id, name, remark, status, cron, app_name, category, url FROM task")
        .unwrap();
    let job_iter = stmt
        .query_map([], |row| {
            Ok(JobModel {
                id: row.get(0)?,
                name: row.get(1)?,
                remark: row.get(2)?,
                status: row.get(3)?,
                cron: row.get(4)?,
                app_name: row.get(5)?,
                category: row.get(6)?,
                url: row.get(7)?,
            })
        })
        .map_err(|_| "数据库查询异常!")?;

    //
    let mut tasks = Vec::new();
    for job in job_iter.into_iter() {
        tasks.push(job.map_err(|_| format!("数据库查询异常!"))?)
    }
    Ok(tasks)
}

pub fn create_job(conn: &Connection, job_in: &JobModel) -> Result<(), String> {
    conn.execute(
        "INSERT INTO task (name, remark, status, cron, app_name, category, url) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (&job_in.name, &job_in.remark, &job_in.status, &job_in.cron, &job_in.app_name, &job_in.category, &job_in.url),
    ).map_err(|e| format!("原因：{}", e))?;
    Ok(())
}
