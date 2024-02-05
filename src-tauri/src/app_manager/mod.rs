mod base_app;
mod schemas;
mod svn_app;
use self::base_app::RepoCommand;
use serde::Serialize;
use svn_app::SvnRepo;
use tauri::Window;

#[derive(Serialize)]
pub struct App {
    name: String,
    url: String,
    category: String,
    description: String,
    status: String,
}
#[derive(Serialize)]
pub struct AppList {
    data: Vec<App>,
    total: u32,
}

#[tauri::command]
pub fn get_app_categories() -> Result<Vec<String>, String> {
    Ok(vec!["分类1".to_string(), "分类2".to_string()])
}

#[tauri::command]
pub fn get_apps(
    app_name: String,
    category: String,
    status: String,
    page: u32,
    items_per_page: u32,
) -> Result<AppList, String> {
    println!("{app_name}-{category}-{status}-{page}-{items_per_page}");
    Ok(AppList {
        data: vec![App {
            name: "应用1".to_string(),
            url: "http://10.1.1.1".to_string(),
            category: "分类1".to_string(),
            description: "test".to_string(),
            status: "已安装".to_string(),
        }],
        total: 1,
    })
}

#[tauri::command]
pub fn ungrade_app() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn install_app(repo_url: String) -> Result<(), String> {
    if repo_url.ends_with(".git") {
        return Err("不支持安装git应用".to_string());
    }
    let svn_repo = SvnRepo::new(repo_url);
    svn_repo.checkout()?;
    svn_repo.install_requirements()
}

#[tauri::command]
pub fn uninstall_app(repo_url: &str) -> Result<(), String> {
    let svn_repo = SvnRepo::new(repo_url.to_string());
    svn_repo.delete()
}

#[tauri::command]
pub fn readme_app(repo_url: &str) -> Result<String, String> {
    let svn_repo: SvnRepo = SvnRepo::new(repo_url.to_string());
    svn_repo.cat("readme.md")
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
