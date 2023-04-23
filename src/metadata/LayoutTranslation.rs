use crate::metadata::LayoutSectionTranslation::LayoutSectionTranslation;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LayoutTranslation  {
	#[serde(rename = "layout")]
	pub layout: String,
	#[serde(rename = "layoutType")]
	pub layout_type: Option<String>,
	#[serde(rename = "sections")]
	pub sections: Option<Vec<LayoutSectionTranslation>>,
}
