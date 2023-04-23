use crate::metadata::Keyword::Keyword;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct KeywordList  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "keywords")]
	pub keywords: Option<Vec<Keyword>>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
}
