use crate::metadata::SvcCatalogFulfillFlowItem::SvcCatalogFulfillFlowItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SvcCatalogFulfillmentFlow  {
	#[serde(rename = "description")]
	pub description: String,
	#[serde(rename = "flow")]
	pub flow: String,
	#[serde(rename = "icon")]
	pub icon: Option<String>,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "items")]
	pub items: Option<Vec<SvcCatalogFulfillFlowItem>>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
