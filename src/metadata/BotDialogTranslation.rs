use crate::metadata::BotStepTranslation::BotStepTranslation;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BotDialogTranslation  {
	#[serde(rename = "botSteps")]
	pub bot_steps: Option<Vec<BotStepTranslation>>,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "label")]
	pub label: Option<String>,
}
