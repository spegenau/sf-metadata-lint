use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EinsteinDealInsightsSettings  {
	#[serde(rename = "enableUnlikelyToCloseThisMonth")]
	pub enable_unlikely_to_close_this_month: Option<bool>,
}
