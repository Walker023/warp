rust_i18n::i18n!("locales", fallback = "en");

pub mod browser;
pub mod fonts;
pub mod platform;
pub mod rendering;
pub mod windowing;

// Re-export everything from the core crate.
pub use warpui_core::*;
