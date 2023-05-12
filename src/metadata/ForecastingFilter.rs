use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ForecastingFilter  {
	#[serde(rename = "filterLogic")]
	pub filter_logic: Option<String>,
	#[serde(rename = "forecastingType")]
	pub forecasting_type: String,
	#[serde(rename = "forecastingTypeSource")]
	pub forecasting_type_source: String,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
