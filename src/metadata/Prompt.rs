use crate::metadata::PromptVersion::PromptVersion;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Prompt  {
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "promptVersions")]
	pub prompt_versions: Option<Vec<PromptVersion>>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
