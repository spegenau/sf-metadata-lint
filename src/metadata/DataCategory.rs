use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DataCategory  {
	#[serde(rename = "dataCategory")]
	pub data_category: Option<Vec<Box<DataCategory>>>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "name")]
	pub name: String,
}
