use crate::metadata::SynonymGroup::SynonymGroup;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SynonymDictionary  {
	#[serde(rename = "groups")]
	pub groups: Option<Vec<SynonymGroup>>,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "label")]
	pub label: String,
}
