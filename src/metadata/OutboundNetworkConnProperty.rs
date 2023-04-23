use crate::metadata::OutboundConnPropertyName::OutboundConnPropertyName;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OutboundNetworkConnProperty  {
	#[serde(rename = "propertyName")]
	pub property_name: OutboundConnPropertyName,
	#[serde(rename = "propertyValue")]
	pub property_value: String,
}
