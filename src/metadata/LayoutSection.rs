use crate::metadata::LayoutColumn::LayoutColumn;
use crate::metadata::LayoutSectionStyle::LayoutSectionStyle;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LayoutSection  {
	#[serde(rename = "customLabel")]
	pub custom_label: Option<bool>,
	#[serde(rename = "detailHeading")]
	pub detail_heading: Option<bool>,
	#[serde(rename = "editHeading")]
	pub edit_heading: Option<bool>,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "layoutColumns")]
	pub layout_columns: Option<Vec<LayoutColumn>>,
	#[serde(rename = "style")]
	pub style: LayoutSectionStyle,
}
