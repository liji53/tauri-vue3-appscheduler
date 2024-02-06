use super::base_app::{url_to_local_path, RepoCommand};
use std::process::Command;

pub struct SvnRepo {
    url: String,        // 远程仓库地址
    local_path: String, // 本地仓库路径
}
impl SvnRepo {
    pub fn new(url: String) -> SvnRepo {
        return SvnRepo {
            local_path: url_to_local_path(&url),
            url,
        };
    }
}
impl RepoCommand for SvnRepo {
    fn local_path(&self) -> &String {
        &self.local_path
    }
    // todo: 用户名和密码 从配置中读取
    fn remote_cat(url: &str) -> Result<String, String> {
        let output = Command::new("svn")
            .arg("cat")
            .arg(url)
            .arg("--non-interactive")
            .arg("--trust-server-cert")
            .output()
            .map_err(|e| format!("原因：{}", e))?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("svn cat失败，原因: {}", stderr);
            Err(format!("原因：{}", stderr))
        }
    }
    // todo: 用户名和密码 从配置中读取
    fn checkout(&self) -> Result<(), String> {
        /* 默认使用本地svn的账号 */
        let output = Command::new("svn")
            .arg("checkout")
            .arg(&self.url)
            .arg(&self.local_path)
            .arg("--non-interactive")
            .arg("--trust-server-cert")
            .output()
            .map_err(|e| format!("原因: {}", e))?;
        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("svn checkout 失败: {}", stderr);
            Err(format!("原因：{}", stderr))
        }
    }
}
