use gpui::IntoElement;

/// 页面布局 trait
///
/// 定义了所有页面组件必须实现的布局方法
/// 用于统一管理不同页面的渲染逻辑
///
/// # 为什么需要 `Self: Sized` 约束？
///
/// 在 Rust 中，trait 中的 `Self` 类型默认是 `?Sized` 的（可能是不定大小的），
/// 这意味着 trait 可以被动态大小类型（DST）实现，比如 `str`、`[T]` 等。
///
/// 但是 `gpui::Context<Self>` 要求 `Self` 必须是 `Sized` 的（大小在编译时已知），
/// 因为 `Context` 需要在栈上分配内存，它的类型参数必须是固定大小的类型。
///
/// 如果不添加 `where Self: Sized` 约束，编译器会报错：
/// "the size for values of type `Self` cannot be known at compilation time"
///
/// 添加 `where Self: Sized` 后，告诉编译器：
/// - 这个 trait 只能被定大小的类型实现
/// - `Self` 的大小在编译时是已知的
/// - 可以安全地用于 `Context<Self>`
pub(crate) trait PageLayout
where
    Self: Sized,
{
    /// 页面布局方法
    ///
    /// # 参数
    /// * `cx` - GPUI 上下文，用于访问窗口、主题等资源
    ///
    /// # 返回
    /// 实现了 `IntoElement` trait 的页面布局元素
    fn page_layout(&mut self, cx: &mut gpui::Context<Self>) -> impl IntoElement;
}
