use crate::metadata::FlowChoiceUserInputTranslation::FlowChoiceUserInputTranslation;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowChoiceTranslation  {
	#[serde(rename = "choiceText")]
	pub choice_text: Option<String>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "userInput")]
	pub user_input: Option<FlowChoiceUserInputTranslation>,
}
