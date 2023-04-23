use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ForecastingObjectListSelectedSettings  {
	#[serde(rename = "field")]
	pub field: Option<Vec<String>>,
}
