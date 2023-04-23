use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AnimationRule  {
	#[serde(rename = "animationFrequency")]
	pub animation_frequency: String,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "isActive")]
	pub is_active: bool,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "recordTypeContext")]
	pub record_type_context: String,
	#[serde(rename = "recordTypeName")]
	pub record_type_name: Option<String>,
	#[serde(rename = "sobjectType")]
	pub sobject_type: String,
	#[serde(rename = "targetField")]
	pub target_field: String,
	#[serde(rename = "targetFieldChangeToValues")]
	pub target_field_change_to_values: String,
}
