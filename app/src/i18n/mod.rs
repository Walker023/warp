// i18n 封装模块。
//
// 注意：`rust_i18n::i18n!()` 宏在 crate 根（`lib.rs`）调用，那里会生成
// `t!` 宏依赖的内部符号。本模块只负责重新导出 API 和提供便捷封装。

// 重新导出 rust-i18n 的常用 API
pub use rust_i18n::{set_locale, t};

/// 获取当前语言
pub fn current_locale() -> String {
    rust_i18n::locale().to_string()
}

/// 设置语言（"en", "zh-CN"）
pub fn switch_locale(locale: &str) {
    set_locale(locale);
}

/// 获取可用语言列表
pub fn get_available_locales() -> Vec<String> {
    rust_i18n::available_locales!()
        .into_iter()
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // 所有翻译相关断言合并到单个测试中串行执行。
    // rust-i18n 的当前 locale 是进程级全局状态，多个测试并行
    // 调用 set_locale 会相互干扰，因此这里集中验证。
    #[test]
    fn test_translations_and_locale() {
        // 切换到英文
        set_locale("en");
        assert_eq!(current_locale(), "en");
        assert_eq!(t!("menu.file"), "File");
        assert_eq!(t!("menu.edit"), "Edit");
        assert_eq!(t!("menu.view"), "View");
        assert_eq!(t!("menu.help"), "Help");

        // 不存在的键应回退为键名本身
        let missing_key = t!("non.existent.key");
        assert!(missing_key.contains("non.existent.key"));

        // 切换到中文
        set_locale("zh-CN");
        assert_eq!(current_locale(), "zh-CN");
        assert_eq!(t!("menu.file"), "文件");
        assert_eq!(t!("menu.edit"), "编辑");
        assert_eq!(t!("menu.view"), "视图");
        assert_eq!(t!("menu.help"), "帮助");

        // 恢复默认英文，避免影响其他测试
        set_locale("en");
    }

    #[test]
    fn test_available_locales() {
        let locales = get_available_locales();
        assert!(locales.iter().any(|l| l == "en"));
        assert!(locales.iter().any(|l| l == "zh-CN"));
    }
}
