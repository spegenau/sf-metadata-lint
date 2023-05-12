use crate::metadata::EnrichedField::EnrichedField;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PlatformEventChannelMember  {
	#[serde(rename = "enrichedFields")]
	pub enriched_fields: Option<Vec<EnrichedField>>,
	#[serde(rename = "eventChannel")]
	pub event_channel: String,
	#[serde(rename = "filterExpression")]
	pub filter_expression: Option<String>,
	#[serde(rename = "selectedEntity")]
	pub selected_entity: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
