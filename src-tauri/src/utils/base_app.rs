use super::schemas::{
    AppStoreConfigProp, AppStoreItem, FormData, FormEntry, FormItemConfig, FormItems, Notice,
    NoticeItem,
};
use super::{is_selectd_componet, task_config_file, task_log_file};
use chrono::{DateTime, Local};
use indexmap::IndexMap;
use ini::Ini;
use std::io::Write;
use std::path::Path;
use std::{env, fs, thread};
use tauri::Window;

// 版本管理工具命令 接口
pub trait RepoCommand {
    fn local_path(&self) -> &String;

    /// 使用版本管理工具，导出仓库(python项目)
    fn checkout(&self) -> Result<(), String>;

    /// 升级本地仓库到最新
    fn update(&self) -> Result<(), String>;

    // 读取远程仓库中指定文件的内容
    fn remote_cat(url: &str) -> Result<String, String>;

    /// 读取本地仓库中指定文件的内容
    fn cat(&self, file_name: &str) -> Result<String, String> {
        let file = Path::new(self.local_path()).join(file_name);
        if !file.exists() {
            return Err("文件不存在".to_string());
        }
        fs::read_to_string(file.to_string_lossy().to_string())
            .map_err(|e| format!("文件读取异常: {}", e))
    }

    /// 删除本地仓库
    fn delete(&self) -> Result<(), String> {
        println!("delete dir: {}", self.local_path());
        fs::remove_dir_all(self.local_path()).map_err(|e| format!("{}", e))
    }

    /// 安装python项目中的依赖库，pip install -r requirements.txt
    fn install_requirements(&self) -> Result<(), String> {
        let requirement_path = Path::new(self.local_path()).join("requirements.txt");
        if !requirement_path.exists() {
            eprintln!("requirements.txt文件不存在");
            return Ok(());
        }
        let requirement_path = requirement_path.to_string_lossy().to_string();

        let output = super::command_warp(vec!["pip", "install", "-r", &requirement_path])?;
        if output.status.success() {
            Ok(())
        } else {
            eprintln!(
                "pip install 失败：{}",
                String::from_utf8_lossy(&output.stderr)
            );
            Err("执行pip install失败".to_string())
        }
    }

    /// 执行应用程序，python main.py
    /// todo: 指定配置参数，任务执行结果记入到数据库中
    fn run_app(
        &self,
        window: Option<Window>,
        task_name: String,
        task_id: u32,
    ) -> Result<(), String> {
        let repo_path = self.local_path().clone();
        let path = Path::new(&repo_path).join("main.py");
        if !path.exists() {
            if let Ok(mut fd) = fs::File::create(Path::new(&repo_path).join(task_log_file(task_id)))
            {
                let _ = fd.write_all("项目中不存在main.py".as_bytes());
            }
            return Err("项目中不存在main.py".to_string());
        }
        // 多线程执行任务
        thread::spawn(move || {
            // 起一个进程执行 python main.py
            let _ = env::set_current_dir(&repo_path);
            let output = super::command_warp(vec!["python", "main.py"]);

            // 解析任务执行的结果
            let mut success = false;
            let description: String;
            let log_content: String;
            match output {
                Ok(ret) => {
                    if ret.status.success() {
                        success = true;
                        description = "任务执行成功".to_string();
                        log_content = String::from_utf8_lossy(&ret.stdout).to_string();
                    } else {
                        description = "任务脚本执行报错".to_string();
                        log_content = String::from_utf8_lossy(&ret.stderr).to_string();
                    }
                }
                Err(e) => {
                    description = e.clone();
                    log_content = e;
                }
            }

            // 通知js，任务执行的结果
            if let Some(window) = window {
                let now: DateTime<Local> = Local::now();
                let formatted_time = now.format("%m-%d %H:%M:%S").to_string();
                window
                .emit(
                    "run_app_result",
                    Notice{
                            name: "任务结果".to_string(),
                            list: vec![NoticeItem {
                                title: task_name,
                                avatar: "https://gw.alipayobjects.com/zos/rmsportal/ThXAXghbEsBCCSDihZxY.png".to_string(),
                                datetime: formatted_time,
                                r#type: "1".to_string(),
                                description,
                                status: if success {"success".to_string()} else {"danger".to_string()},
                                extra: if success {"成功".to_string()} else {"失败".to_string()},
                            }],
                        }
                )
                .unwrap();
            }

            // 记入日志
            if let Ok(mut fd) = fs::File::create(Path::new(&repo_path).join(task_log_file(task_id)))
            {
                let _ = fd.write_all(log_content.as_bytes());
            }
        });

        Ok(())
    }

    /// 获取当前任务的配置(注：配置的section会忽略，不能存在相同的key，如有相同的key最后的生效)
    /// todo: 支持多section、考虑应用升级的情况
    fn get_config(&self, app_store_item: AppStoreItem, task_id: u32) -> Result<String, String> {
        // 如果有设置过配置，则用该任务的配置，没有则用默认配置
        let mut config_path = Path::new(self.local_path()).join(task_config_file(task_id));
        if !config_path.exists() {
            config_path = Path::new(self.local_path()).join("config.ini");
        }
        if !config_path.exists() {
            return Err("不存在配置文件".to_string());
        }
        let mut raw_config = IndexMap::new();
        let ini = Ini::load_from_file(config_path).map_err(|_| "配置文件解析失败!".to_string())?;
        for (_section, prop) in ini.iter() {
            for (k, v) in prop.iter() {
                raw_config.insert(k.to_string(), v.to_string());
            }
        }

        // 将配置文件内容转为指定格式的数据
        let mut config_form: Vec<FormEntry> = Vec::new();
        let mut id = 0;
        for (key, value) in raw_config.iter() {
            id += 1;
            // 找项目配置中是否有对应app 表单配置
            let mut app_store_config_prop: Option<&AppStoreConfigProp> = None;
            if app_store_item.config.is_some() {
                app_store_config_prop = app_store_item
                    .config
                    .as_ref()
                    .unwrap()
                    .iter()
                    .find(|prop| prop.field_name == key.as_str());
            }

            // 格式转化
            if app_store_config_prop.is_none() {
                // 没有，则使用默认的表单(Text)
                config_form.push(FormEntry {
                    control_type: "Text".to_string(),
                    id: id.to_string(),
                    data: FormData {
                        field_name: key.to_string(),
                        label: key.to_string(),
                        required: false,
                        default: Some(value.to_string()),
                        tip: None,
                        placeholder: None,
                        min: None,
                        max: None,
                        item_config: None,
                    },
                });
            } else {
                //
                let prop = app_store_config_prop.unwrap();
                let items = prop.option_items.clone().unwrap_or([].to_vec());
                let items = items
                    .iter()
                    .enumerate()
                    .map(|(idx, item)| FormItems {
                        id: idx.to_string() + "1",
                        label: item.to_string(),
                        value: item.to_string(),
                    })
                    .collect::<Vec<_>>();

                config_form.push(FormEntry {
                    control_type: prop.control_type.clone(),
                    id: id.to_string(),
                    data: FormData {
                        field_name: key.to_string(),
                        label: key.to_string(),
                        required: prop.required.is_some_and(|x| x),
                        tip: prop.tip.clone(),
                        placeholder: prop.placeholder.clone(),
                        min: prop.min.clone(),
                        max: prop.max.clone(),
                        default: Some(value.to_string()), // 非select组件才有效
                        item_config: if is_selectd_componet(&prop.control_type) {
                            // select 有效
                            Some(FormItemConfig {
                                value: value.to_string(),
                                items,
                            })
                        } else {
                            None
                        },
                    },
                });
            }
        }
        Ok(serde_json::to_string(&config_form)
            .map_err(|_| "form struct转json str失败!".to_string())?)
    }

    /// 保存配置
    /// todo: 同get_config
    fn set_config(&self, config_in: serde_json::Value, task_id: u32) -> Result<(), String> {
        let config_in = config_in.as_object().unwrap();
        let task_config_path = Path::new(self.local_path()).join(task_config_file(task_id));
        let mut config_path = task_config_path.clone();
        if !config_path.exists() {
            config_path = Path::new(self.local_path()).join("config.ini");
        }

        // 更新数据
        let cur_config =
            Ini::load_from_file(config_path).map_err(|e| format!("文件解析失败: {e}"))?;
        let mut new_config = Ini::new();
        for (section, prop) in cur_config.iter() {
            for (k, v) in prop.iter() {
                if config_in.contains_key(k) {
                    new_config
                        .with_section(Some(section.unwrap()))
                        .set(k, config_in[k].as_str().unwrap_or(""));
                } else {
                    new_config.with_section(Some(section.unwrap())).set(k, v);
                }
            }
        }
        // 写入当前任务的配置文件
        new_config
            .write_to_file(task_config_path)
            .map_err(|e| format!("文件保存失败：{e}"))
    }
}
