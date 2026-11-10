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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SettingsStore;
    use gpui::TestAppContext;

    #[gpui::test]
    fn ui_language_setting_updates(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let mut store = SettingsStore::new(cx, &crate::default_settings());
            store.register_setting::<LanguageSettings>();

            store
                .set_user_settings(r#"{"ui_language":"en"}"#, cx)
                .unwrap();
            assert_eq!(
                store.get::<LanguageSettings>(None).language,
                i18n::Language::English
            );

            store
                .set_user_settings(r#"{"ui_language":"zh-CN"}"#, cx)
                .unwrap();
            assert_eq!(
                store.get::<LanguageSettings>(None).language,
                i18n::Language::ChineseSimplified
            );
        });
    }
}
