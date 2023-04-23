use crate::metadata::KnowledgeLanguage::KnowledgeLanguage;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct KnowledgeLanguageSettings  {
	#[serde(rename = "language")]
	pub language: Option<Vec<KnowledgeLanguage>>,
}
