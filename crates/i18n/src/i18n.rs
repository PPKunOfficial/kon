//! Translation support for Kon's user interface.
//!
//! Strings are looked up by their English source text, which means call sites
//! can opt into localization incrementally. Any string that has not been
//! translated yet is shown in English.

use std::sync::atomic::{AtomicU8, Ordering};

mod zh_cn;
mod zh_cn_descriptions;
mod zh_cn_settings;

/// A language supported by Kon's user interface.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum Language {
    /// English (the source language).
    #[default]
    English,
    /// Simplified Chinese.
    ChineseSimplified,
}

impl Language {
    /// Resolves a language from a `ui_language` setting value.
    ///
    /// `None`, `"system"`, and unknown values fall back to the system locale.
    pub fn from_setting(value: Option<&str>) -> Self {
        match value.map(str::trim) {
            None | Some("") | Some("system") => Self::from_system_locale(),
            Some("en" | "en-US" | "en_US") => Self::English,
            Some("zh" | "zh-CN" | "zh_CN" | "zh-Hans" | "zh-Hans-CN" | "zh-SG") => {
                Self::ChineseSimplified
            }
            Some(_) => Self::from_system_locale(),
        }
    }

    /// Detects the language from the operating system locale.
    pub fn from_system_locale() -> Self {
        sys_locale::get_locale()
            .as_deref()
            .map(Self::from_locale)
            .unwrap_or_default()
    }

    /// Resolves a language from a locale string such as `zh-CN`.
    pub fn from_locale(locale: &str) -> Self {
        if locale.to_ascii_lowercase().starts_with("zh") {
            Self::ChineseSimplified
        } else {
            Self::English
        }
    }

    /// Returns the BCP 47 code for this language.
    pub fn code(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::ChineseSimplified => "zh-CN",
        }
    }

    /// Returns the name of this language as shown in the settings UI.
    pub fn display_name(self) -> &'static str {
        match self {
            Self::English => "English",
            Self::ChineseSimplified => "简体中文",
        }
    }
}

static CURRENT_LANGUAGE: AtomicU8 = AtomicU8::new(Language::English as u8);

/// Sets the language used for subsequent translations.
pub fn set_language(language: Language) {
    CURRENT_LANGUAGE.store(language as u8, Ordering::Relaxed);
}

/// Returns the language currently used for translations.
pub fn language() -> Language {
    match CURRENT_LANGUAGE.load(Ordering::Relaxed) {
        value if value == Language::ChineseSimplified as u8 => Language::ChineseSimplified,
        _ => Language::English,
    }
}

/// Translates a static English string into the current language.
///
/// Untranslated strings are returned unchanged.
pub fn tr(text: &'static str) -> &'static str {
    lookup(text).unwrap_or(text)
}

/// Looks up a translation for a string in the current language.
pub fn lookup(text: &str) -> Option<&'static str> {
    match language() {
        Language::English => None,
        Language::ChineseSimplified => zh_cn::lookup(text)
            .or_else(|| zh_cn_settings::lookup(text))
            .or_else(|| zh_cn_descriptions::lookup(text)),
    }
}

#[cfg(test)]
mod tests {
    use super::Language;

    #[test]
    fn resolves_language_codes() {
        assert_eq!(Language::from_locale("zh-CN"), Language::ChineseSimplified);
        assert_eq!(Language::from_locale("zh_TW"), Language::ChineseSimplified);
        assert_eq!(Language::from_locale("en-US"), Language::English);
        assert_eq!(Language::from_locale("de-DE"), Language::English);
    }

    #[test]
    fn resolves_setting_values() {
        assert_eq!(
            Language::from_setting(Some("zh-CN")),
            Language::ChineseSimplified
        );
        assert_eq!(Language::from_setting(Some("en")), Language::English);
    }

    #[test]
    fn has_simplified_chinese_translations() {
        assert_eq!(super::zh_cn::lookup("File"), Some("文件"));
        assert_eq!(super::zh_cn::lookup("Open Settings"), Some("打开设置"));
        assert_eq!(super::zh_cn::lookup("Not a translated string"), None);
    }

    #[test]
    fn has_settings_translations() {
        assert_eq!(super::zh_cn_settings::lookup("Font Size"), Some("字体大小"));
        assert_eq!(
            super::zh_cn_descriptions::lookup("Show breadcrumbs."),
            Some("显示面包屑导航。")
        );
        assert_eq!(super::zh_cn_settings::lookup("Not a setting"), None);
    }

    #[test]
    fn translates_static_strings_in_chinese() {
        super::set_language(Language::ChineseSimplified);
        assert_eq!(super::tr("File"), "文件");
        super::set_language(Language::English);
        assert_eq!(super::tr("File"), "File");
    }
}
