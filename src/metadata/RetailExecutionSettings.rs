use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RetailExecutionSettings  {
	#[serde(rename = "enableProductHierarchy")]
	pub enable_product_hierarchy: Option<bool>,
	#[serde(rename = "enableRetailExecution")]
	pub enable_retail_execution: Option<bool>,
	#[serde(rename = "enableVisitSharing")]
	pub enable_visit_sharing: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
