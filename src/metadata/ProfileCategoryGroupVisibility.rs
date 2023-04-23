use crate::metadata::CategoryGroupVisibility::CategoryGroupVisibility;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ProfileCategoryGroupVisibility  {
	#[serde(rename = "dataCategories")]
	pub data_categories: Option<Vec<String>>,
	#[serde(rename = "dataCategoryGroup")]
	pub data_category_group: String,
	#[serde(rename = "visibility")]
	pub visibility: CategoryGroupVisibility,
}
