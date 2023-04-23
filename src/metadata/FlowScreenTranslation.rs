use crate::metadata::FlowScreenFieldTranslation::FlowScreenFieldTranslation;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowScreenTranslation  {
	#[serde(rename = "backButtonLabel")]
	pub back_button_label: Option<String>,
	#[serde(rename = "fields")]
	pub fields: Option<Vec<FlowScreenFieldTranslation>>,
	#[serde(rename = "helpText")]
	pub help_text: Option<String>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "nextOrFinishButtonLabel")]
	pub next_or_finish_button_label: Option<String>,
	#[serde(rename = "pauseButtonLabel")]
	pub pause_button_label: Option<String>,
	#[serde(rename = "pausedText")]
	pub paused_text: Option<String>,
}
