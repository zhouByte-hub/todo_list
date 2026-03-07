use anyhow::anyhow;
use gpui::{AssetSource, Result, SharedString};
use rust_embed::RustEmbed;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "src/assets/"]
#[include = "*.svg"]
#[include = "*.png"]
pub(crate) struct TodoIconAssets;

impl AssetSource for TodoIconAssets {
    /// 加载指定路径的图标文件
    ///
    /// # 参数
    /// * `path` - 图标文件的路径（相对于嵌入的文件夹）
    ///
    /// # 返回
    /// * `Ok(Some(bytes))` - 成功加载图标文件的字节数据
    /// * `Ok(None)` - 路径为空
    /// * `Err(e)` - 加载失败（文件不存在）
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        Self::get(path)
            .map(|f| Some(f.data))
            .ok_or_else(|| anyhow!("could not find asset at path \"{path}\""))
    }

    /// 列出指定路径下的所有图标文件
    ///
    /// # 参数
    /// * `path` - 要列出的路径前缀
    ///
    /// # 返回
    /// * `Ok(Vec)` - 匹配路径前缀的所有文件名列表
    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect())
    }
}
