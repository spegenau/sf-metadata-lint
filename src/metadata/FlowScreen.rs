use crate::metadata::FlowConnector::FlowConnector;
use crate::metadata::FlowScreenField::FlowScreenField;
use crate::metadata::FlowScreenRule::FlowScreenRule;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowScreen  {
	#[serde(rename = "allowBack")]
	pub allow_back: Option<bool>,
	#[serde(rename = "allowFinish")]
	pub allow_finish: Option<bool>,
	#[serde(rename = "allowPause")]
	pub allow_pause: Option<bool>,
	#[serde(rename = "backButtonLabel")]
	pub back_button_label: Option<String>,
	#[serde(rename = "connector")]
	pub connector: Option<FlowConnector>,
	#[serde(rename = "fields")]
	pub fields: Option<Vec<FlowScreenField>>,
	#[serde(rename = "helpText")]
	pub help_text: Option<String>,
	#[serde(rename = "nextOrFinishButtonLabel")]
	pub next_or_finish_button_label: Option<String>,
	#[serde(rename = "pauseButtonLabel")]
	pub pause_button_label: Option<String>,
	#[serde(rename = "pausedText")]
	pub paused_text: Option<String>,
	#[serde(rename = "rules")]
	pub rules: Option<Vec<FlowScreenRule>>,
	#[serde(rename = "showFooter")]
	pub show_footer: Option<bool>,
	#[serde(rename = "showHeader")]
	pub show_header: Option<bool>,
}
