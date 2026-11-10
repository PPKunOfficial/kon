use crate::{self as settings, settings_content::UILanguageContent};
use i18n::Language;
use settings::{RegisterSetting, Settings};

/// The language used by Kon's user interface.
#[derive(Copy, Clone, Debug, RegisterSetting)]
pub struct LanguageSettings {
    pub language: Language,
}

impl Settings for LanguageSettings {
    fn from_settings(s: &crate::settings_content::SettingsContent) -> Self {
        let language = match s.ui_language {
            Some(UILanguageContent::English) => Language::English,
            Some(UILanguageContent::ChineseSimplified) => Language::ChineseSimplified,
            Some(UILanguageContent::System) | None => Language::from_system_locale(),
        };
        Self { language }
    }
}
