use crate::metadata::SynonymGroup::SynonymGroup;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MlSlotClassValue  {
	#[serde(rename = "synonymGroup")]
	pub synonym_group: Option<SynonymGroup>,
	#[serde(rename = "synonymGroups")]
	pub synonym_groups: Option<Vec<SynonymGroup>>,
	#[serde(rename = "value")]
	pub value: String,
}
