use crate::metadata::FlowCategoryItems::FlowCategoryItems;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowCategory  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "flowCategoryItems")]
	pub flow_category_items: Option<Vec<FlowCategoryItems>>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
