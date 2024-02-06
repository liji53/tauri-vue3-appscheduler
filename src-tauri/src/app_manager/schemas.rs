#[derive(serde::Serialize)]
pub struct App {
    pub name: String,
    pub url: String,
    pub category: String,
    pub description: String,
    pub status: String,
}
#[derive(serde::Serialize)]
pub struct AppList {
    pub data: Vec<App>,
    pub total: u32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AppStoreItem {
    pub name: String,
    pub category: String,
    pub url: String,
    pub online: Option<bool>,
    pub remark: Option<String>,
    pub config: Option<serde_json::Value>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AppStoreConfig {
    pub app_list: Vec<AppStoreItem>,
}

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
/// 返回给前端的payload
#[derive(Clone, serde::Serialize)]
pub struct RunAppPayload {
    pub notice: Notice,
    pub log: Log,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct FormData {
    #[serde(rename = "fieldName")]
    pub field_name: String,
    pub label: String,
    pub tip: String,
    pub placeholder: String,
    #[serde(rename = "showRule")]
    pub show_rule: String,
    pub required: bool,
    pub rule: String,
    pub default: String,
    pub csslist: String,
}
/// 应用程序的配置表单
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct FormItem {
    #[serde(rename = "ControlType")]
    pub control_type: String,
    #[serde(rename = "nameCn")]
    pub name_cn: String,
    pub id: String,
    pub layout: bool,
    pub data: FormData,
}
