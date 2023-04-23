use crate::metadata::BotStep::BotStep;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BotDialog  {
	#[serde(rename = "botDialogGroup")]
	pub bot_dialog_group: Option<String>,
	#[serde(rename = "botSteps")]
	pub bot_steps: Option<Vec<BotStep>>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "isPlaceholderDialog")]
	pub is_placeholder_dialog: Option<bool>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "mlIntent")]
	pub ml_intent: Option<String>,
	#[serde(rename = "mlIntentTrainingEnabled")]
	pub ml_intent_training_enabled: Option<bool>,
	#[serde(rename = "showInFooterMenu")]
	pub show_in_footer_menu: Option<bool>,
}
