use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConversationRecordLookupField  {
	#[serde(rename = "fieldName")]
	pub field_name: String,
}
