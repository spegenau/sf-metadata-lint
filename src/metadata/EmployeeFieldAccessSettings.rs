use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmployeeFieldAccessSettings  {
	#[serde(rename = "enableEmployeeFieldMaskDefaults")]
	pub enable_employee_field_mask_defaults: Option<bool>,
	#[serde(rename = "enableEmployeeFieldMasking")]
	pub enable_employee_field_masking: Option<bool>,
}
