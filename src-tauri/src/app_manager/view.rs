use super::schemas::{App, AppPagination, AppTree, AppTreeChildren};
use crate::utils;
use crate::utils::base_app::RepoCommand;
use crate::utils::schemas::AppStoreItem;
use crate::utils::svn_app::SvnRepo;
use std::collections::{HashMap, HashSet};

const INSTALLED: &str = "已安装";
const UNINSTALLED: &str = "未安装";

/// 获取应用分类
#[tauri::command]
pub fn get_app_categories() -> Result<Vec<String>, String> {
    let error = "获取应用分类失败";
    // 读取远程仓库中的配置文件
    let app_config = utils::cached_app_store_config().map_err(|e| format!("{}, {}", error, e))?;

    // 获取应用的分类
    let mut categories = HashSet::new();
    for item in app_config.app_list.into_iter() {
        if item.online.is_some_and(|online| !online) {
            continue;
        }
        categories.insert(item.category);
    }
    let categories: Vec<String> = categories.into_iter().collect();
    Ok(categories)
}

/// 获取应用列表
#[tauri::command]
pub fn get_apps(
    app_name: String,
    category: String,
    status: String,
    page: u32,
    items_per_page: u32,
) -> Result<AppPagination, String> {
    let error = "获取应用列表失败";
    let app_config = utils::cached_app_store_config().map_err(|e| format!("{}, {}", error, e))?;

    // 基于入参过滤
    let app_store_list: Vec<AppStoreItem> = app_config
        .app_list
        .into_iter()
        .filter(|item| {
            (item.online.is_none() || item.online.is_some_and(|x| x)) // 配置了在线
                && (app_name.is_empty() || item.name.contains(&app_name))
                && (category.is_empty() || item.category == category)
                && (status.is_empty()
                    || (utils::exists_project(&item.url)
                        == (if status == INSTALLED { true } else { false })))
        })
        .collect();

    let app_store_list = utils::paginate(app_store_list, page, items_per_page);
    let apps: Vec<App> = app_store_list
        .into_iter()
        .map(|i| App {
            name: i.name,
            status: if utils::exists_project(&i.url) {
                String::from(INSTALLED)
            } else {
                String::from(UNINSTALLED)
            },
            url: i.url,
            category: i.category,
            description: i.remark.unwrap_or("".to_string()),
        })
        .collect();

    Ok(AppPagination {
        total: apps.len() as u32,
        data: apps,
    })
}

/// 安装应用，下载远程应用到本地
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

/// 卸载应用
#[tauri::command]
pub fn uninstall_app(repo_url: String) -> Result<(), String> {
    let svn_repo = SvnRepo::new(repo_url);
    svn_repo
        .delete()
        .map_err(|e| format!("卸载应用失败, {}", e))
}

/// 升级应用
#[tauri::command]
pub fn ungrade_app(repo_url: String) -> Result<(), String> {
    let svn_repo = SvnRepo::new(repo_url);
    svn_repo
        .update()
        .map_err(|e| format!("升级应用失败, {}", e))
}

/// 获取应用的详情(readme.md)
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

/// 基于分类-应用的层次，返回前端el-select-tree要求的树形结构(应用管理中用到)
#[tauri::command]
pub fn get_app_tree() -> Result<Vec<AppTree>, String> {
    let error = "获取应用分类树失败";
    // 读取远程仓库中的配置文件
    let app_config = utils::cached_app_store_config().map_err(|e| format!("{}, {}", error, e))?;
    // 获取已经安装的应用，format：{应用分类: [安装的应用名]}
    let mut app_categories = HashMap::new();
    for item in app_config.app_list.into_iter() {
        if !utils::exists_project(&item.url) || item.online.is_some_and(|online| !online) {
            // 应用没有安装,或者下线了
            continue;
        }
        if !app_categories.contains_key(&item.category) {
            app_categories.insert(item.category, vec![item.name]);
        } else {
            app_categories
                .get_mut(&item.category)
                .unwrap()
                .push(item.name)
        }
    }
    // 格式转化
    let mut ret = Vec::new();
    for item in app_categories {
        let tmp: Vec<AppTreeChildren> = item
            .1
            .into_iter()
            .map(|app_name| AppTreeChildren { name: app_name })
            .collect();
        ret.push(AppTree {
            name: item.0,
            children: tmp,
        });
    }
    Ok(ret)
}
