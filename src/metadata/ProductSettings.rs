use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ProductSettings  {
	#[serde(rename = "enableCascadeActivateToRelatedPrices")]
	pub enable_cascade_activate_to_related_prices: Option<bool>,
	#[serde(rename = "enableMySettings")]
	pub enable_my_settings: Option<bool>,
	#[serde(rename = "enableQuantitySchedule")]
	pub enable_quantity_schedule: Option<bool>,
	#[serde(rename = "enableRevenueSchedule")]
	pub enable_revenue_schedule: Option<bool>,
}
