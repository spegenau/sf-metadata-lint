use crate::metadata::ProductAttributeSetItem::ProductAttributeSetItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ProductAttributeSet  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "productAttributeSetItems")]
	pub product_attribute_set_items: Option<Vec<ProductAttributeSetItem>>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
