use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowTextTemplateTranslation  {
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "text")]
	pub text: Option<String>,
}
