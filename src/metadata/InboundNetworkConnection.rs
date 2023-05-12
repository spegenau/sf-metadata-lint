use crate::metadata::ExternalConnectionStatus::ExternalConnectionStatus;
use crate::metadata::ExternalConnectionType::ExternalConnectionType;
use crate::metadata::InboundNetworkConnProperty::InboundNetworkConnProperty;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct InboundNetworkConnection  {
	#[serde(rename = "connectionType")]
	pub connection_type: ExternalConnectionType,
	#[serde(rename = "description")]
	pub description: String,
	#[serde(rename = "inboundNetworkConnProperties")]
	pub inbound_network_conn_properties: Option<Vec<InboundNetworkConnProperty>>,
	#[serde(rename = "isActive")]
	pub is_active: bool,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "status")]
	pub status: ExternalConnectionStatus,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
