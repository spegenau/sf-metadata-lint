use crate::metadata::ItemActionType::ItemActionType;
use crate::metadata::ItemCategory::ItemCategory;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ActionLauncherItemDef  {
	#[serde(rename = "identifier")]
	pub identifier: String,
	#[serde(rename = "itemActionType")]
	pub item_action_type: ItemActionType,
	#[serde(rename = "itemCategory")]
	pub item_category: ItemCategory,
	#[serde(rename = "itemLanguage")]
	pub item_language: String,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "subType")]
	pub sub_type: String,
	#[serde(rename = "type")]
	pub _type: String,
	#[serde(rename = "versionNumber")]
	pub version_number: String,
}
