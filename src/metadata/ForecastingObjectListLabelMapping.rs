use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ForecastingObjectListLabelMapping  {
	#[serde(rename = "field")]
	pub field: String,
	#[serde(rename = "label")]
	pub label: String,
}
