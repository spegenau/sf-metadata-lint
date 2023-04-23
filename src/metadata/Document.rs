use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Document  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "internalUseOnly")]
	pub internal_use_only: bool,
	#[serde(rename = "keywords")]
	pub keywords: Option<String>,
	#[serde(rename = "name")]
	pub name: Option<String>,
	#[serde(rename = "public")]
	pub public: bool,
}
