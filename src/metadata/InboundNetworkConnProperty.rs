use crate::metadata::InboundConnPropertyName::InboundConnPropertyName;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct InboundNetworkConnProperty  {
	#[serde(rename = "propertyName")]
	pub property_name: InboundConnPropertyName,
	#[serde(rename = "propertyValue")]
	pub property_value: String,
}
