use crate::metadata::RecordActionType::RecordActionType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RecordActionSelectableItem  {
	#[serde(rename = "action")]
	pub action: String,
	#[serde(rename = "frequentActionSequenceNbr")]
	pub frequent_action_sequence_nbr: Option<i32>,
	#[serde(rename = "isFrequentAction")]
	pub is_frequent_action: Option<bool>,
	#[serde(rename = "type")]
	pub _type: RecordActionType,
}
