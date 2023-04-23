use crate::metadata::WeightedSourceCategory::WeightedSourceCategory;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ForecastingCategoryMapping  {
	#[serde(rename = "forecastingItemCategoryApiName")]
	pub forecasting_item_category_api_name: String,
	#[serde(rename = "weightedSourceCategories")]
	pub weighted_source_categories: Option<Vec<WeightedSourceCategory>>,
}
