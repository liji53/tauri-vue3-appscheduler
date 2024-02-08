/// 在应用商城中，用于展示应用列表的元数据

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AppStoreItem {
    pub name: String,
    pub category: String,
    pub url: String, // 由于没有用数据库设计，因此该字段作为唯一ID，作用类似于 数据库的主键
    pub online: Option<bool>,
    pub remark: Option<String>,
    pub config: Option<serde_json::Value>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct AppStoreConfig {
    pub app_list: Vec<AppStoreItem>,
}

/// 与JS通信相关的数据结构
#[derive(Clone, serde::Serialize)]
pub struct NoticeItem {
    pub avatar: String,
    pub title: String,
    pub datetime: String,
    pub r#type: String,
    pub description: String,
    pub status: String,
    pub extra: String,
}
/// 用于通知栏的数据格式
#[derive(Clone, serde::Serialize)]
pub struct Notice {
    pub name: String,
    pub list: Vec<NoticeItem>,
}
/// 服务器后端保存的日志格式
#[derive(Clone, serde::Serialize)]
pub struct Log {
    pub status: bool,
    pub execute_type: String,
    pub content: String,
    pub task_id: u32,
}
