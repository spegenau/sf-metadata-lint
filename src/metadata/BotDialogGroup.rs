use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BotDialogGroup  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "label")]
	pub label: String,
}
