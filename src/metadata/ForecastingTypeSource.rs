use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ForecastingTypeSource  {
	#[serde(rename = "forecastingSourceDefinition")]
	pub forecasting_source_definition: String,
	#[serde(rename = "forecastingType")]
	pub forecasting_type: String,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "parentSourceDefinition")]
	pub parent_source_definition: Option<String>,
	#[serde(rename = "relationField")]
	pub relation_field: Option<String>,
	#[serde(rename = "sourceGroup")]
	pub source_group: i32,
}
