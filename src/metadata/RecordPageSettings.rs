use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RecordPageSettings  {
	#[serde(rename = "enableActivityRelatedList")]
	pub enable_activity_related_list: Option<bool>,
	#[serde(rename = "enableFullRecordView")]
	pub enable_full_record_view: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
