use serde::{Deserialize, Serialize};
use settings::macros::define_settings_group;
use settings::{RespectUserSyncSetting, SupportedPlatforms, SyncToCloud};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "kebab-case")]
#[schemars(
    description = "The application UI language.",
    rename_all = "kebab-case"
)]
pub enum Locale {
    #[default]
    #[schemars(description = "English")]
    En,
    #[schemars(description = "Simplified Chinese")]
    ZhCn,
}

impl Locale {
    pub const ALL: [Self; 2] = [Self::En, Self::ZhCn];

    /// Convert to rust-i18n locale string.
    pub fn to_str(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::ZhCn => "zh-CN",
        }
    }

    /// Display name in the locale's own language.
    pub fn display_name(self) -> &'static str {
        match self {
            Self::En => "English",
            Self::ZhCn => "简体中文",
        }
    }
}

impl settings_value::SettingsValue for Locale {}

define_settings_group!(LanguageSettings, settings: [
    locale: LanguageLocale {
        type: Locale,
        default: Locale::En,
        supported_platforms: SupportedPlatforms::DESKTOP,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        private: false,
        storage_key: "Language",
        toml_path: "general.language",
        description: "The application UI language. Restart Warp for changes to take effect.",
    },
]);

#[cfg(test)]
mod tests {
    use serde_json::json;
    use settings_value::SettingsValue as _;

    use super::Locale;

    #[test]
    fn locale_runtime_and_settings_file_values_are_stable() {
        assert_eq!(Locale::En.to_str(), "en");
        assert_eq!(Locale::ZhCn.to_str(), "zh-CN");
        assert_eq!(Locale::En.display_name(), "English");
        assert_eq!(Locale::ZhCn.display_name(), "简体中文");

        assert_eq!(Locale::En.to_file_value(), json!("en"));
        assert_eq!(Locale::ZhCn.to_file_value(), json!("zh-cn"));
        assert_eq!(Locale::from_file_value(&json!("en")), Some(Locale::En));
        assert_eq!(Locale::from_file_value(&json!("zh-cn")), Some(Locale::ZhCn));
        assert_eq!(Locale::from_file_value(&json!("zh-CN")), None);
    }
}
