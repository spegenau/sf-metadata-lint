use crate::metadata::MlIntent::MlIntent;
use crate::metadata::MlSlotClass::MlSlotClass;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MlDomain  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "mlIntents")]
	pub ml_intents: Option<Vec<MlIntent>>,
	#[serde(rename = "mlSlotClasses")]
	pub ml_slot_classes: Option<Vec<MlSlotClass>>,
}
