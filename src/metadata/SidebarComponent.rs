use crate::metadata::RelatedList::RelatedList;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SidebarComponent  {
	#[serde(rename = "componentType")]
	pub component_type: String,
	#[serde(rename = "createAction")]
	pub create_action: Option<String>,
	#[serde(rename = "enableLinking")]
	pub enable_linking: Option<bool>,
	#[serde(rename = "height")]
	pub height: Option<i32>,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "lookup")]
	pub lookup: Option<String>,
	#[serde(rename = "page")]
	pub page: Option<String>,
	#[serde(rename = "relatedLists")]
	pub related_lists: Option<Vec<RelatedList>>,
	#[serde(rename = "unit")]
	pub unit: Option<String>,
	#[serde(rename = "updateAction")]
	pub update_action: Option<String>,
	#[serde(rename = "width")]
	pub width: Option<i32>,
}
