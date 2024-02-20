/// 项目配置文件，用于在应用商城中，展示应用列表的元数据
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AppStoreConfigProp {
    pub field_name: String, // 任务配置的key
    pub tip: Option<String>,
    pub placeholder: Option<String>,
    pub required: Option<bool>,
    pub min: Option<i32>, // Slider
    pub max: Option<i32>, // Slider
    // "Text" | "TextArea" | "Switch" | "Radio" | "CheckBox" | "Date" | "Time" | "InputNumber" | "Slider" | "Selected" | "Selecteds"
    pub control_type: String,
    pub option_items: Option<Vec<String>>, // "Radio", "CheckBox", "Selected", "Selecteds" 才有
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AppStoreItem {
    pub name: String,
    pub category: String,
    pub url: String, // 由于没有用数据库设计，因此该字段作为唯一ID，作用类似于 数据库的主键
    pub online: Option<bool>,
    pub remark: Option<String>,
    pub config: Option<Vec<AppStoreConfigProp>>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct AppStoreConfig {
    pub app_list: Vec<AppStoreItem>,
}

/// 任务配置的表单格式
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct FormItems {
    pub id: String,
    pub label: String,
    pub value: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct FormItemConfig {
    pub items: Vec<FormItems>, // 选项
    pub value: String,         // 当前值
}
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct FormData {
    #[serde(rename = "fieldName")]
    pub field_name: String,
    pub label: String,
    pub tip: Option<String>,
    pub placeholder: Option<String>,
    pub required: bool,
    pub min: Option<i32>,        // Slider
    pub max: Option<i32>,        // Slider
    pub default: Option<String>, // text等，与item_config互斥
    #[serde(rename = "itemConfig")]
    pub item_config: Option<FormItemConfig>, // select等
}
/// 应用程序的配置表单(单项)
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct FormEntry {
    #[serde(rename = "ControlType")]
    pub control_type: String,
    pub id: String,
    pub data: FormData,
}

/// 与JS通信相关的数据结构(通知栏的数据结构)
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
