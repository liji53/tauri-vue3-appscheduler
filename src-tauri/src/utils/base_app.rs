use std::fs;
use std::path::Path;
use std::process::Command;

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
}
