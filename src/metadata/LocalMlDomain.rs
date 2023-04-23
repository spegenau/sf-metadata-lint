use crate::metadata::MlIntent::MlIntent;
use crate::metadata::MlSlotClass::MlSlotClass;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LocalMlDomain  {
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "mlIntents")]
	pub ml_intents: Option<Vec<MlIntent>>,
	#[serde(rename = "mlSlotClasses")]
	pub ml_slot_classes: Option<Vec<MlSlotClass>>,
	#[serde(rename = "name")]
	pub name: String,
}
