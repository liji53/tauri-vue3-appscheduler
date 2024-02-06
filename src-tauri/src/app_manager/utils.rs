use super::RepoCommand;
use super::SvnRepo;
use cached::proc_macro::once;

const CONFIG_URL: &str = "https://192.168.57.30/secu/dep1/UftdbSett/trunk/Documents/D5.Others/产品质量提升工具脚本/S6_项目辅助类工具/config.json";

/// 获取应用配置，并缓存60s
#[once(time = 60, sync_writes = true)]
pub fn cached_app_store_config() -> Result<String, String> {
    SvnRepo::remote_cat(CONFIG_URL)
}

// 分页
pub fn paginate<T>(data: &[T], page: u32, items_per_page: u32) -> Vec<&T> {
    let start = ((page - 1) * items_per_page) as usize;
    // 计算结束索引，但不超过数据总长度
    let end = std::cmp::min(start + items_per_page as usize, data.len());
    data[start..end].iter().collect()
}
