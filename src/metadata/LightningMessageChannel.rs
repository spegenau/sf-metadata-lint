use crate::metadata::LightningMessageField::LightningMessageField;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LightningMessageChannel  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "isExposed")]
	pub is_exposed: Option<bool>,
	#[serde(rename = "lightningMessageFields")]
	pub lightning_message_fields: Option<Vec<LightningMessageField>>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
}
