pub mod base_app;
pub mod database;
pub mod git_app;
pub mod scheduler;
pub mod schemas;
pub mod svn_app;

use self::base_app::RepoCommand;
use self::schemas::{AppStoreConfig, ProgramConfig};
use self::svn_app::SvnRepo;
use cached::proc_macro::cached;

use sha1::{Digest, Sha1};
use std::os::windows::process::CommandExt;
use std::process::{Command, Output};
use std::{fs, path::PathBuf};
use tauri::api::path;

/// 获取应用配置
pub fn get_app_store_config() -> Result<AppStoreConfig, String> {
    let contents =
        fs::read_to_string(program_config_path()).map_err(|_| "本程序的配置还未设置!")?;
    let program_config: ProgramConfig = serde_json::from_str(&contents).unwrap();
    cached_app_store_config(program_config.apps_url)
}

// 缓存应用配置项
#[cached(time = 60, sync_writes = true)]
fn cached_app_store_config(url: String) -> Result<AppStoreConfig, String> {
    if !url.ends_with(".git") {
        let repo = SvnRepo::new(url);
        let app_store_config = repo.remote_cat(None)?;
        let ret: AppStoreConfig =
            serde_json::from_str(&app_store_config).map_err(|_| "配置文件json格式异常!")?;
        Ok(ret)
    } else {
        Err("暂不支持git仓库!".to_string())
    }
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
    // Windows: Resolves to {FOLDERID_LocalAppData}.
    let local_data_dir = path::local_data_dir().unwrap();
    let data_path = local_data_dir.join("appscheduler");
    if !data_path.exists() {
        fs::create_dir_all(&data_path).unwrap()
    }
    data_path
}

/// 本程序的配置文件
fn program_config_path() -> String {
    program_data_path()
        .join("config.json")
        .to_string_lossy()
        .to_string()
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
    [
        "Radio",
        "CheckBox",
        "Selected",
        "Selecteds",
        "Files",
        "File",
    ]
    .contains(&componet.as_str())
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
