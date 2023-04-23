use crate::metadata::EmbeddedServiceFlowType::EmbeddedServiceFlowType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmbeddedServiceFlow  {
	#[serde(rename = "flow")]
	pub flow: String,
	#[serde(rename = "flowType")]
	pub flow_type: EmbeddedServiceFlowType,
	#[serde(rename = "isAuthenticationRequired")]
	pub is_authentication_required: bool,
}
