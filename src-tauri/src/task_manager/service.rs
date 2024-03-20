use super::schemas::{JobModel, JobUpdate};
use crate::utils;
use rusqlite::{Connection, Params, Result, ToSql};

fn query<P>(conn: &Connection, sql_stmt: &str, params: P) -> Result<Vec<JobModel>, String>
where
    P: Params,
{
    let mut stmt = conn.prepare(sql_stmt).unwrap();
    let job_iter = stmt
        .query_map(params, |row| {
            Ok(JobModel {
                id: row.get(0)?,
                name: row.get(1)?,
                remark: row.get(2)?,
                status: row.get(3)?,
                cron: row.get(4)?,
                app_name: row.get(5)?,
                category: row.get(6)?,
                url: row.get(7)?,
                pre_success: row.get(8)?,
            })
        })
        .map_err(|_| "数据库查询异常!")?;

    let mut jobs = Vec::new();
    for job in job_iter {
        jobs.push(job.map_err(|e| format!("数据库查询异常: {}", e))?)
    }
    Ok(jobs)
}

pub fn get_all(conn: &Connection) -> Result<Vec<JobModel>, String> {
    let sql_stmt =
        "SELECT id, name, remark, status, cron, app_name, category, url, pre_success FROM task";
    query(conn, sql_stmt, [])
}

pub fn get_by_id(conn: &Connection, id: u32) -> Result<Option<JobModel>, String> {
    let sql_stmt =
        "SELECT id, name, remark, status, cron, app_name, category, url, pre_success FROM task WHERE id = ?1";
    let jobs = query(conn, sql_stmt, [id])?;
    if jobs.len() == 0 {
        Ok(None) // 没有找到指定ID的Task
    } else {
        Ok(Some(jobs.into_iter().next().unwrap()))
    }
}

pub fn get_by_name(conn: &Connection, name: &String) -> Result<Option<JobModel>, String> {
    let sql_stmt =
        "SELECT id, name, remark, status, cron, app_name, category, url, pre_success FROM task WHERE name = ?1";
    let jobs = query(conn, sql_stmt, [name])?;
    if jobs.len() == 0 {
        Ok(None) // 没有找到指定ID的Task
    } else {
        Ok(Some(jobs.into_iter().next().unwrap()))
    }
}

pub fn get_by_url(conn: &Connection, url: &String) -> Result<Vec<JobModel>, String> {
    let sql_stmt =
        "SELECT id, name, remark, status, cron, app_name, category, url, pre_success FROM task WHERE url = ?1";
    query(conn, sql_stmt, [url])
}

pub fn create(conn: &Connection, job_in: &JobModel) -> Result<(), String> {
    let sql = "INSERT INTO task (name, remark, status, cron, app_name, category, url, pre_success) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)";
    let params = (
        &job_in.name,
        &job_in.remark,
        &job_in.status,
        &job_in.cron,
        &job_in.app_name,
        &job_in.category,
        &job_in.url,
        &job_in.pre_success,
    );
    conn.execute(sql, params)
        .map_err(|e| format!("原因：{}", e))
        .map(|_| ()) // 如果成功，我们不需要返回特定的值，只需确认操作成功即可
}

pub fn delete(conn: &Connection, id: u32) -> Result<(), String> {
    let sql = "DELETE FROM task WHERE id = ?1";
    conn.execute(sql, [id])
        .map_err(|e| format!("原因: {e}"))
        .map(|_| ()) // 如果成功，我们不需要返回特定的值，只需确认操作成功即可
}

/// 生成sql语句与参数的宏
macro_rules! process_field {
    ($job_update:ident, $field:ident, $parts:ident, $values:ident, $index:ident) => {
        if let Some(ref $field) = $job_update.$field {
            $parts.push(format!("{} = ?{}", stringify!($field), $index));
            $values.push(Box::new($field.clone()));
            $index += 1;
        }
    };
}

// 生成sql更新语句 和 对应的参数
fn generate_update_sql_and_params(job_update: &JobUpdate) -> Option<(String, Vec<Box<dyn ToSql>>)> {
    let mut parts = Vec::new();
    let mut values: Vec<Box<dyn ToSql>> = Vec::new();
    let mut index = 1;

    process_field!(job_update, name, parts, values, index);
    process_field!(job_update, remark, parts, values, index);
    process_field!(job_update, status, parts, values, index);
    process_field!(job_update, cron, parts, values, index);
    process_field!(job_update, pre_success, parts, values, index);
    if let Some(ref app_name) = job_update.app_name {
        if let Ok(app_config) = utils::get_app_store_config() {
            if let Some(app_store_item) = app_config
                .app_list
                .iter()
                .find(|item| item.name == app_name.as_str())
            {
                parts.push(format!("app_name = ?{}", index));
                values.push(Box::new(app_name.clone()));
                index += 1;

                parts.push(format!("category = ?{}", index));
                values.push(Box::new(app_store_item.category.clone()));
                index += 1;

                parts.push(format!("url = ?{}", index));
                values.push(Box::new(app_store_item.url.clone()));
                index += 1;
            }
        }
    }

    if !parts.is_empty() {
        let sql = format!("UPDATE task SET {} WHERE id = ?{}", parts.join(", "), index);
        Some((sql, values))
    } else {
        None
    }
}

pub fn update(conn: &Connection, id: u32, job_in: &JobUpdate) -> Result<(), String> {
    if let Some((sql, params)) = generate_update_sql_and_params(job_in) {
        let mut params_with_id = params;
        params_with_id.push(Box::new(id));

        conn.execute(&sql, rusqlite::params_from_iter(params_with_id))
            .map_err(|e| format!("原因:{e}"))?;
        Ok(())
    } else {
        Err("客户端参数异常!".to_string())
    }
}
