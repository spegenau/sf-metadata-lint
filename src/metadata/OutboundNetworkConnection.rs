use crate::metadata::ExternalConnectionStatus::ExternalConnectionStatus;
use crate::metadata::ExternalConnectionType::ExternalConnectionType;
use crate::metadata::OutboundNetworkConnProperty::OutboundNetworkConnProperty;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OutboundNetworkConnection  {
	#[serde(rename = "connectionType")]
	pub connection_type: ExternalConnectionType,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "isActive")]
	pub is_active: bool,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "outboundNetworkConnProperties")]
	pub outbound_network_conn_properties: Option<Vec<OutboundNetworkConnProperty>>,
	#[serde(rename = "status")]
	pub status: ExternalConnectionStatus,
}
