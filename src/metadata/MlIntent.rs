use crate::metadata::MlIntentUtterance::MlIntentUtterance;
use crate::metadata::MlRelatedIntent::MlRelatedIntent;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MlIntent  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "mlIntentUtterances")]
	pub ml_intent_utterances: Option<Vec<MlIntentUtterance>>,
	#[serde(rename = "relatedMlIntents")]
	pub related_ml_intents: Option<Vec<MlRelatedIntent>>,
}
