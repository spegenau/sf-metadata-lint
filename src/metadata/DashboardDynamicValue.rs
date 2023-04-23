use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DashboardDynamicValue  {
	#[serde(rename = "additionalInfo")]
	pub additional_info: Option<String>,
	#[serde(rename = "fieldName")]
	pub field_name: String,
	#[serde(rename = "isDynamicUser")]
	pub is_dynamic_user: Option<bool>,
}
