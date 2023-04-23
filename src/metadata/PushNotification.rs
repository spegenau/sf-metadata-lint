use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PushNotification  {
	#[serde(rename = "fieldNames")]
	pub field_names: Option<Vec<String>>,
	#[serde(rename = "objectName")]
	pub object_name: String,
}
