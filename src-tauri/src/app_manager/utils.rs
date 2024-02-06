use super::RepoCommand;
use super::SvnRepo;
use cached::proc_macro::once;
use sha1::{Digest, Sha1};

const CONFIG_URL: &str = "https://192.168.57.30/secu/dep1/UftdbSett/trunk/Documents/D5.Others/产品质量提升工具脚本/S6_项目辅助类工具/config.json";

/// 获取应用配置，并缓存60s
#[once(time = 60, sync_writes = true)]
pub fn cached_app_store_config() -> Result<String, String> {
    SvnRepo::remote_cat(CONFIG_URL)
}

/// 对Vec数据进行分页
pub fn paginate<T>(data: &[T], page: u32, items_per_page: u32) -> Vec<&T> {
    let start = ((page - 1) * items_per_page) as usize;
    // 计算结束索引，但不超过数据总长度
    let end = std::cmp::min(start + items_per_page as usize, data.len());
    data[start..end].iter().collect()
}

/// 将仓库地址转化为本地安装路径
pub fn url_to_local_path(url: &String) -> String {
    let mut hasher = Sha1::new();
    hasher.update(url.as_bytes());
    let result = hasher.finalize();

    // 在windows中项目位于C:\USERS\XXX\.appscheduler 目录下
    let mut user_path = ".".to_string();
    if let Ok(user_profile) = std::env::var("USERPROFILE") {
        user_path = user_profile;
    }
    let path = std::path::Path::new(user_path.as_str())
        .join(".appscheduler")
        .join(format!("{:x}", result));
    return path.to_string_lossy().to_string();
}

/// 检查本地是否已经存在指定URL的项目
pub fn exists_project(url: &String) -> bool {
    let local_path = url_to_local_path(url);
    std::path::Path::new(&local_path).exists()
}
