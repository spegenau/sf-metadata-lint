use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WarrantyLifecycleMgmtSettings  {
	#[serde(rename = "enableWarrantyLCMgmt")]
	pub enable_warranty_lc_mgmt: Option<bool>,
}
