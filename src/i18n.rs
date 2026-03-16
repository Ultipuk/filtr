use fluent_templates::fluent_bundle::FluentValue;
use fluent_templates::{Loader as _, static_loader};
use std::borrow::Cow;
use std::collections::HashMap;
use std::str::FromStr as _;
use std::sync::RwLock;
use unic_langid::{LanguageIdentifier, langid};

static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US",
    };
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum UiLanguage {
    #[default]
    System,
    English,
    Russian,
}


static CURRENT_LANGUAGE: RwLock<LanguageIdentifier> = RwLock::new(langid!("en-US"));

pub fn set_language(selection: UiLanguage) {
    let resolved = match selection {
        UiLanguage::System => detect_system_language(),
        UiLanguage::English => langid!("en-US"),
        UiLanguage::Russian => langid!("ru-RU"),
    };

    if let Ok(mut lang) = CURRENT_LANGUAGE.write() {
        *lang = resolved;
    }
}

pub fn tr(key: &str) -> String {
    let lang = CURRENT_LANGUAGE
        .read()
        .map(|v| v.clone())
        .unwrap_or_else(|_| langid!("en-US"));

    LOCALES.lookup(&lang, key)
}

pub fn tr_args(key: &str, args: &[(&str, FluentValue<'_>)]) -> String {
    let lang = CURRENT_LANGUAGE
        .read()
        .map(|v| v.clone())
        .unwrap_or_else(|_| langid!("en-US"));

    let mut fargs: HashMap<Cow<'static, str>, FluentValue<'_>> = HashMap::new();
    for (name, value) in args {
        fargs.insert(Cow::Owned((*name).to_owned()), value.clone());
    }

    LOCALES.lookup_with_args(&lang, key, &fargs).clone()
}

#[cfg(not(target_arch = "wasm32"))]
fn detect_system_locale() -> String {
    sys_locale::get_locale().unwrap_or_else(|| "en-US".to_owned())
}

#[cfg(target_arch = "wasm32")]
fn detect_system_locale() -> String {
    web_sys::window()
        .map(|w| {
            w.navigator()
                .language()
                .unwrap_or_else(|| "en-US".to_owned())
        })
        .unwrap_or_else(|| "en-US".to_owned())
}

fn detect_system_language() -> LanguageIdentifier {
    parse_locale_to_supported(&detect_system_locale())
}

fn parse_locale_to_supported(locale: &str) -> LanguageIdentifier {
    if let Ok(parsed) = LanguageIdentifier::from_str(locale) {
        if parsed.language == langid!("ru").language {
            return langid!("ru-RU");
        }
    }

    if locale.to_ascii_lowercase().starts_with("ru") {
        langid!("ru-RU")
    } else {
        langid!("en-US")
    }
}
