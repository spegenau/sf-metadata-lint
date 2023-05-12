use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LanguageSettings  {
	#[serde(rename = "enableCanadaIcuFormat")]
	pub enable_canada_icu_format: Option<bool>,
	#[serde(rename = "enableDataTranslation")]
	pub enable_data_translation: Option<bool>,
	#[serde(rename = "enableEndUserLanguages")]
	pub enable_end_user_languages: Option<bool>,
	#[serde(rename = "enableICULocaleDateFormat")]
	pub enable_icu_locale_date_format: Option<bool>,
	#[serde(rename = "enableLocalNamesForStdObjects")]
	pub enable_local_names_for_std_objects: Option<bool>,
	#[serde(rename = "enableLocaleInsensitiveFiltering")]
	pub enable_locale_insensitive_filtering: Option<bool>,
	#[serde(rename = "enablePlatformLanguages")]
	pub enable_platform_languages: Option<bool>,
	#[serde(rename = "enableTranslationWorkbench")]
	pub enable_translation_workbench: Option<bool>,
	#[serde(rename = "useLanguageFallback")]
	pub use_language_fallback: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
