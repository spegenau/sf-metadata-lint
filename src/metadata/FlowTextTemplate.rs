use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowTextTemplate  {
	#[serde(rename = "isViewedAsPlainText")]
	pub is_viewed_as_plain_text: Option<bool>,
	#[serde(rename = "text")]
	pub text: String,
}
