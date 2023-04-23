use crate::metadata::LayoutSectionStyle::LayoutSectionStyle;
use crate::metadata::QuickActionLayoutColumn::QuickActionLayoutColumn;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct QuickActionLayout  {
	#[serde(rename = "layoutSectionStyle")]
	pub layout_section_style: LayoutSectionStyle,
	#[serde(rename = "quickActionLayoutColumns")]
	pub quick_action_layout_columns: Option<Vec<QuickActionLayoutColumn>>,
}
