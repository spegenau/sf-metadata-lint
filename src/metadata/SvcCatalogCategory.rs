use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SvcCatalogCategory  {
	#[serde(rename = "image")]
	pub image: Option<String>,
	#[serde(rename = "isActive")]
	pub is_active: Option<bool>,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "parentCategory")]
	pub parent_category: Option<String>,
	#[serde(rename = "sortOrder")]
	pub sort_order: Option<i32>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
