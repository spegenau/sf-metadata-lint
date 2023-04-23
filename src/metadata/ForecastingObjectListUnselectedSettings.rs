use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ForecastingObjectListUnselectedSettings  {
	#[serde(rename = "field")]
	pub field: Option<Vec<String>>,
}
