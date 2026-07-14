rust_i18n::i18n!("locales", fallback = "en");

pub mod content;
pub mod decoration;
pub mod editor;
pub mod model;
pub mod multiline;
mod parallel_util;
pub mod render;
pub mod search;
pub mod selection;
