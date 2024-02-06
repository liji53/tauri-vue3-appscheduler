mod base_app;
mod schemas;
mod svn_app;
mod utils;
use self::base_app::RepoCommand;
use schemas::{App, AppList, AppStoreConfig, AppStoreItem};
use std::collections::HashSet;
use svn_app::SvnRepo;
use tauri::Window;

const INSTALLED: &str = "已安装";
const UNINSTALLED: &str = "未安装";

#[tauri::command]
pub fn get_app_categories() -> Result<Vec<String>, String> {
    let error = "获取应用分类失败";
    // 读取远程仓库中的配置文件
    let content = utils::cached_app_store_config().map_err(|e| format!("{}, {}", error, e))?;
    let app_config: AppStoreConfig = serde_json::from_str(&content)
        .map_err(|_| format!("{}, {}", error, "配置文件json格式异常!"))?;

    // 获取应用的分类
    let mut categories = HashSet::new();
    for item in app_config.app_list.into_iter() {
        categories.insert(item.category);
    }
    let categories: Vec<String> = categories.into_iter().collect();
    Ok(categories)
}

#[tauri::command]
pub fn get_apps(
    app_name: String,
    category: String,
    status: String,
    page: u32,
    items_per_page: u32,
) -> Result<AppList, String> {
    let error = "获取应用列表失败";
    let content = utils::cached_app_store_config().map_err(|e| format!("{}, {}", error, e))?;
    let app_config: AppStoreConfig = serde_json::from_str(&content)
        .map_err(|_| format!("{}, {}", error, "配置文件json格式异常!"))?;

    // 基于入参过滤
    let app_store_list: Vec<AppStoreItem> = app_config
        .app_list
        .into_iter()
        .filter(|item| {
            (app_name.is_empty() || item.name.contains(&app_name))
                && (category.is_empty() || item.category == category)
                && (status.is_empty()
                    || (utils::exists_project(&item.url)
                        == (if status == INSTALLED { true } else { false })))
        })
        .collect();

    let app_store_list = utils::paginate(&app_store_list, page, items_per_page);
    let apps: Vec<App> = app_store_list
        .into_iter()
        .map(|i| App {
            name: i.name.to_string(),
            url: i.url.to_string(),
            category: i.category.to_string(),
            description: i.remark.to_owned().unwrap_or("".to_string()),
            status: if utils::exists_project(&i.url) {
                String::from(INSTALLED)
            } else {
                String::from(UNINSTALLED)
            },
        })
        .collect();

    Ok(AppList {
        total: apps.len() as u32,
        data: apps,
    })
}

#[tauri::command]
pub fn install_app(repo_url: String) -> Result<(), String> {
    if repo_url.ends_with(".git") {
        return Err("不支持安装git应用".to_string());
    }
    let svn_repo = SvnRepo::new(repo_url);
    svn_repo
        .checkout()
        .map_err(|e| format!("安装应用失败, {}", e))?;
    svn_repo
        .install_requirements()
        .map_err(|e| format!("安装应用成功, 但{}", e))
}

#[tauri::command]
pub fn uninstall_app(repo_url: String) -> Result<(), String> {
    let svn_repo = SvnRepo::new(repo_url);
    svn_repo
        .delete()
        .map_err(|e| format!("卸载应用失败, {}", e))
}

#[tauri::command]
pub fn ungrade_app() -> Result<(), String> {
    Err("尚未实现".to_string())
}

#[tauri::command]
pub fn readme_app(repo_url: String) -> Result<String, String> {
    let error = "获取详情失败";
    if !utils::exists_project(&repo_url) {
        let remote_file = format!("{}/{}", repo_url, "readme.md");
        SvnRepo::remote_cat(&remote_file).map_err(|e| format!("{}, {}", error, e))
    } else {
        let svn_repo: SvnRepo = SvnRepo::new(repo_url.to_string());
        svn_repo
            .cat("readme.md")
            .map_err(|e| format!("{}, {}", error, e))
    }
}

#[tauri::command]
pub fn run_app(
    window: Window,
    repo_url: String,
    task_name: String,
    task_id: u32,
) -> Result<(), String> {
    let svn_repo: SvnRepo = SvnRepo::new(repo_url);
    svn_repo.run_app(window, task_name, task_id)
}

#[tauri::command]
pub fn getconfig_app(repo_url: String, app_form: String, task_id: u32) -> Result<String, String> {
    let svn_repo: SvnRepo = SvnRepo::new(repo_url);
    svn_repo.getconfig_app(app_form, task_id)
}

#[tauri::command]
pub fn setconfig_app(repo_url: String, config: String, task_id: u32) -> Result<(), String> {
    let svn_repo: SvnRepo = SvnRepo::new(repo_url);
    svn_repo.setconfig_app(config, task_id)
}
