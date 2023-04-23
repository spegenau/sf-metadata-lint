use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct InventorySettings  {
	#[serde(rename = "enableOCIB2CIntegration")]
	pub enable_ocib_2_c_integration: Option<bool>,
	#[serde(rename = "enableOmniChannelInventory")]
	pub enable_omni_channel_inventory: Option<bool>,
}
