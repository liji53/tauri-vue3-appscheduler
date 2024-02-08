/// 与JS通信相关的数据结构
#[derive(serde::Serialize)]
pub struct Job {
    pub id: u32,
    pub name: String,
    pub remark: String,
    pub status: bool, // 是否启用
    pub next_at: String,
    pub cron: String,

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

/// DB models
pub struct JobModel {
    pub id: Option<u32>,
    pub name: String,
    pub remark: String,
    pub status: bool, // 是否启用
    pub cron: String,

    pub app_name: String,
    pub category: String,
    pub url: String,
}
