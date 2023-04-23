use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PermissionSetFlowAccess  {
	#[serde(rename = "enabled")]
	pub enabled: bool,
	#[serde(rename = "flow")]
	pub flow: String,
}
