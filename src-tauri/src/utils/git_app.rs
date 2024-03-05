use super::base_app::RepoCommand;
use super::url_to_local_path;
use git2::{Error, Repository};
use std::path::Path;

pub struct GitRepo {
    url: String,        // 远程仓库地址
    local_path: String, // 本地仓库路径
    branch: String,
}
impl GitRepo {
    pub fn new(url: String) -> GitRepo {
        return GitRepo {
            local_path: url_to_local_path(&url),
            url,
            branch: "master".to_string(),
        };
    }

    fn pull(&self) -> Result<(), Error> {
        let repo = Repository::open(&self.local_path)?;

        repo.find_remote("origin")?
            .fetch(&[&self.branch], None, None)?;

        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
        let analysis = repo.merge_analysis(&[&fetch_commit])?;
        if analysis.0.is_up_to_date() {
            Ok(())
        } else if analysis.0.is_fast_forward() {
            let refname = format!("refs/heads/{}", self.branch);
            let mut reference = repo.find_reference(&refname)?;
            reference.set_target(fetch_commit.id(), "Fast-Forward")?;
            repo.set_head(&refname)?;
            repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
        } else {
            Err(Error::from_str("Fast-forward only!"))
        }
    }
}

impl RepoCommand for GitRepo {
    fn local_path(&self) -> &String {
        &self.local_path
    }

    fn remote_cat(&self, file_path: Option<&str>) -> Result<String, String> {
        let file_path = file_path.unwrap();
        if Path::exists(Path::new(self.local_path())) {
            self.update()?;
        } else {
            self.checkout()?;
        }
        self.cat(file_path)
    }

    fn update(&self) -> Result<(), String> {
        self.pull().map_err(|e| format!("原因：{e}"))
    }

    fn checkout(&self) -> Result<(), String> {
        Repository::clone(&self.url, self.local_path()).map_err(|e| format!("原因：{e}"))?;
        Ok(())
    }
}
