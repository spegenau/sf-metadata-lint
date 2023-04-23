use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WeightedSourceCategory  {
	#[serde(rename = "sourceCategoryApiName")]
	pub source_category_api_name: String,
	#[serde(rename = "weight")]
	pub weight: f32,
}
