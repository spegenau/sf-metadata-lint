use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ForecastingDisplayedFamilySettings  {
	#[serde(rename = "productFamily")]
	pub product_family: Option<String>,
}
