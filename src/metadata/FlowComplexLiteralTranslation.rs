use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowComplexLiteralTranslation  {
	#[serde(rename = "customAspectKey")]
	pub custom_aspect_key: Option<String>,
	#[serde(rename = "value")]
	pub value: Option<String>,
}
