use crate::metadata::SummaryLayoutItem::SummaryLayoutItem;
use crate::metadata::SummaryLayoutStyle::SummaryLayoutStyle;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SummaryLayout  {
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "sizeX")]
	pub size_x: i32,
	#[serde(rename = "sizeY")]
	pub size_y: Option<i32>,
	#[serde(rename = "sizeZ")]
	pub size_z: Option<i32>,
	#[serde(rename = "summaryLayoutItems")]
	pub summary_layout_items: Option<Vec<SummaryLayoutItem>>,
	#[serde(rename = "summaryLayoutStyle")]
	pub summary_layout_style: SummaryLayoutStyle,
}
