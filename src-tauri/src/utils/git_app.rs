use super::base_app::RepoCommand;
use super::url_to_local_path;
use git2::Repository;

pub struct GitRepo {
    url: String,        // 远程仓库地址
    local_path: String, // 本地仓库路径
}
impl GitRepo {
    pub fn new(url: String) -> GitRepo {
        return GitRepo {
            local_path: url_to_local_path(&url),
            url,
        };
    }
}

// todo: 用户名和密码 从配置中读取
impl RepoCommand for GitRepo {
    fn local_path(&self) -> &String {
        &self.local_path
    }

    fn remote_cat(&self, file_path: Option<&str>) -> Result<String, String> {
        Ok("".to_string())
    }

    fn update(&self) -> Result<(), String> {
        Ok(())
    }

    fn checkout(&self) -> Result<(), String> {
        let repo = Repository::clone(&self.url, &self.local_path);
        Ok(())
    }
}
