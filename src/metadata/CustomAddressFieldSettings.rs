use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomAddressFieldSettings  {
	#[serde(rename = "enableCustomAddressField")]
	pub enable_custom_address_field: Option<bool>,
}
