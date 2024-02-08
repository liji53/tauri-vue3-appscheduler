/// 在应用商城中，用于展示应用列表的元数据

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AppStoreItem {
    pub name: String,
    pub category: String,
    pub url: String,
    pub online: Option<bool>,
    pub remark: Option<String>,
    pub config: Option<serde_json::Value>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct AppStoreConfig {
    pub app_list: Vec<AppStoreItem>,
}
