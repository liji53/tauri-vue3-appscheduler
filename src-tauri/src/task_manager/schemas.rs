/// 与JS通信相关的数据结构
#[derive(serde::Serialize)]
pub struct Job {
    pub id: u32,
    pub name: String,
    pub remark: String,
    pub status: bool, // 是否启用
    pub next_at: Option<String>,
    pub cron: String,

    pub pre_success: Option<bool>, // 上次执行是否成功
    pub app_name: String,
    pub category: String,
    pub url: String,
}

#[derive(serde::Serialize)]
pub struct JobPagination {
    pub total: u32,
    pub data: Vec<Job>,
}

#[derive(serde::Deserialize)]
pub struct JobCreate {
    pub name: String,
    pub remark: String,
    pub app_name: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct JobUpdate {
    pub name: Option<String>,
    pub remark: Option<String>,
    pub app_name: Option<String>,
    pub status: Option<bool>,
    pub cron: Option<String>,
    pub pre_success: Option<bool>, // 上一次是否执行成功
}

#[derive(serde::Serialize)]
pub struct JobLog {
    pub created_at: String, // 执行时间
    pub content: String,
}

#[derive(serde::Serialize)]
pub struct JobResult {
    pub created_at: String, // 执行时间
    pub html_path: String,
}

/// DB models
#[derive(Debug)]
pub struct JobModel {
    pub id: Option<u32>,
    pub name: String,
    pub remark: String,
    pub status: bool, // 是否启用
    pub cron: String,

    pub pre_success: Option<bool>, // 上一次是否执行成功
    pub app_name: String,
    pub category: String,
    pub url: String,
}
