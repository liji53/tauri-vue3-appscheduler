use crate::utils::base_app::RepoCommand;
use crate::utils::git_app::GitRepo;
use crate::utils::svn_app::SvnRepo;
use crate::utils::url_to_local_path;

/// 获取git/svn 仓库
pub fn get_repo(url: String) -> Box<dyn RepoCommand> {
    if url.ends_with(".git") {
        return Box::new(GitRepo::new(url));
    } else {
        return Box::new(SvnRepo::new(url));
    }
}

/// 检查本地是否已经存在指定URL的应用
pub fn exists_app(url: &String) -> bool {
    let local_path = url_to_local_path(url);
    std::path::Path::new(&local_path).exists()
}
