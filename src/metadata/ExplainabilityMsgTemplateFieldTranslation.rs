use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ExplainabilityMsgTemplateFieldTranslation  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "templateMessage")]
	pub template_message: Option<String>,
}
