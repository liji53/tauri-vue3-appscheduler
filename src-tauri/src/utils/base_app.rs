use super::schemas::{Notice, NoticeItem};
use chrono::{DateTime, Local};
use std::path::Path;
use std::process::Command;
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
    fn cat(&self, file_path: &str) -> Result<String, String> {
        let file = Path::new(self.local_path()).join(file_path);
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
        let output = Command::new("pip")
            .arg("install")
            .arg("-r")
            .arg(format!("{}", requirement_path.to_string_lossy()))
            .output()
            .map_err(|e| format!("执行pip install失败: {}", e))?;
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
    fn run_app(&self, window: Window, task_name: String, task_id: u32) -> Result<(), String> {
        let repo_path = self.local_path().clone();
        let path = Path::new(&repo_path).join("main.py");
        if !path.exists() {
            return Err("项目中不存在main.py".to_string());
        }
        // 多线程执行任务
        thread::spawn(move || {
            // 起一个进程执行 python main.py
            let _ = env::set_current_dir(repo_path);
            let output = Command::new("python")
                .arg("main.py")
                .output()
                .map_err(|e| format!("执行python main.py报错: {}", e));

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

            println!("{task_id} - {log_content}")
            // todo: 记入日志
            // log: Log{
            //     status: success,
            //     execute_type: "手动".to_string(),
            //     content: log_content,
            //     task_id
            // }
        });

        Ok(())
    }
}
