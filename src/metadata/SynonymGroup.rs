use crate::metadata::Language::Language;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SynonymGroup  {
	#[serde(rename = "languages")]
	pub languages: Option<Vec<Language>>,
	#[serde(rename = "terms")]
	pub terms: Option<Vec<String>>,
}
