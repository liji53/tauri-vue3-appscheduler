use super::base_app::RepoCommand;
use super::url_to_local_path;

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

// todo: 用户名和密码 从配置中读取
impl RepoCommand for SvnRepo {
    fn local_path(&self) -> &String {
        &self.local_path
    }

    fn remote_cat(&self, file_path: Option<&str>) -> Result<String, String> {
        let file_uri = match file_path {
            Some(file) => format!("{}/{}", self.url, file),
            None => self.url.clone(),
        };
        let output = super::command_warp(vec![
            "svn",
            "cat",
            &file_uri,
            "--non-interactive",
            "--trust-server-cert",
        ])?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("svn cat失败，原因: {}", stderr);
            Err(format!("原因：{}", stderr))
        }
    }

    fn update(&self) -> Result<(), String> {
        let output = super::command_warp(vec![
            "svn",
            "update",
            &self.local_path,
            "--non-interactive",
            "--trust-server-cert",
        ])?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("svn update失败，原因: {}", stderr);
            Err(format!("原因：{}", stderr))
        }
    }

    fn checkout(&self) -> Result<(), String> {
        /* 默认使用本地svn的账号 */
        let output = super::command_warp(vec![
            "svn",
            "checkout",
            &self.url,
            &self.local_path,
            "--non-interactive",
            "--trust-server-cert",
        ])?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("svn checkout 失败: {}", stderr);
            Err(format!("原因：{}", stderr))
        }
    }
}
