use crate::metadata::BotDialogTranslation::BotDialogTranslation;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BotVersionTranslation  {
	#[serde(rename = "botDialogs")]
	pub bot_dialogs: Option<Vec<BotDialogTranslation>>,
	#[serde(rename = "fullName")]
	pub full_name: String,
}
