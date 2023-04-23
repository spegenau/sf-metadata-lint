use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AdjustmentsSettings  {
	#[serde(rename = "enableAdjustments")]
	pub enable_adjustments: bool,
	#[serde(rename = "enableOwnerAdjustments")]
	pub enable_owner_adjustments: bool,
}
