use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Group  {
	#[serde(rename = "doesIncludeBosses")]
	pub does_include_bosses: Option<bool>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
