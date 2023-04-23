use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct IndexField  {
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "sortDirection")]
	pub sort_direction: String,
}
