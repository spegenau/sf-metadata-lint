use crate::metadata::PromptVersionTranslation::PromptVersionTranslation;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PromptTranslation  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "promptVersions")]
	pub prompt_versions: Option<Vec<PromptVersionTranslation>>,
}
