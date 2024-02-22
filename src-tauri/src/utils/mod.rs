pub mod base_app;
pub mod database;
pub mod scheduler;
pub mod schemas;
pub mod svn_app;

use self::base_app::RepoCommand;
use self::schemas::AppStoreConfig;
use self::svn_app::SvnRepo;
use cached::proc_macro::{cached, once};

use sha1::{Digest, Sha1};
use std::os::windows::process::CommandExt;
use std::process::{Command, Output};
use std::{fs, path::PathBuf};

const CONFIG_URL: &str = "https://192.168.57.30/secu/dep1/UftdbSett/trunk/Documents/D5.Others/产品质量提升工具脚本/S6_项目辅助类工具/config.json";

/// 获取应用配置，并缓存60s
#[once(time = 60, sync_writes = true)]
pub fn cached_app_store_config() -> Result<AppStoreConfig, String> {
    // 必须显示指定数据类型(编译器不报错)，否则json解析转struct失败
    let ret: AppStoreConfig = serde_json::from_str(&SvnRepo::remote_cat(CONFIG_URL)?)
        .map_err(|_| "配置文件json格式异常!")?;
    Ok(ret)
}

/// 对Vec数据进行分页
pub fn paginate<T>(data: Vec<T>, page: u32, items_per_page: u32) -> Vec<T> {
    let start = ((page - 1) * items_per_page) as usize;
    // 计算结束索引，但不超过数据总长度
    let end = std::cmp::min(start + items_per_page as usize, data.len());
    data.into_iter().skip(start).take(end - start).collect()
    //data[start..end].iter().collect()
}

/// 本程序的数据目录(包括安装应用时下载的项目、db等)
#[cached]
fn program_data_path() -> PathBuf {
    // 在windows中项目位于C:\USERS\XXX\.appscheduler 目录下
    let mut user_path = ".".to_string();
    if let Ok(user_profile) = std::env::var("USERPROFILE") {
        user_path = user_profile;
    }
    let data_path = std::path::Path::new(&user_path).join(".appscheduler");
    if !data_path.exists() {
        fs::create_dir_all(&data_path).unwrap()
    }
    data_path
}

/// 本程序的数据库文件
pub fn program_db_path() -> String {
    program_data_path()
        .join("appscheduler.db")
        .to_string_lossy()
        .to_string()
}

/// 将仓库地址转化为本地安装路径
pub fn url_to_local_path(url: &String) -> String {
    let mut hasher = Sha1::new();
    hasher.update(url.as_bytes());
    let result = hasher.finalize();

    let path = program_data_path().join(format!("{:x}", result));
    return path.to_string_lossy().to_string();
}

/// 检查本地是否已经存在指定URL的项目
pub fn exists_project(url: &String) -> bool {
    let local_path = url_to_local_path(url);
    std::path::Path::new(&local_path).exists()
}

/// 指定任务的日志文件名
pub fn task_log_file(task_id: u32) -> String {
    format!(".Log_{task_id}.txt")
}

/// 指定任务的配置文件名
pub fn task_config_file(task_id: u32) -> String {
    format!("conf_{task_id}.ini")
}

/// 是否属于选择性的表单组件
pub fn is_selectd_componet(componet: &String) -> bool {
    ["Radio", "CheckBox", "Selected", "Selecteds"].contains(&componet.as_str())
}

/// 执行cmd命令
pub fn command_warp(args: Vec<&str>) -> Result<Output, String> {
    let commnd_args = args.iter().skip(1).collect::<Vec<_>>();
    Command::new(args[0])
        .creation_flags(0x08000000) // 执行时不会出现窗口
        .args(commnd_args)
        .output()
        .map_err(|e| format!("{}执行异常：{}", args[0], e))
}
