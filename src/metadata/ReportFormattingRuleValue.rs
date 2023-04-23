use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReportFormattingRuleValue  {
	#[serde(rename = "backgroundColor")]
	pub background_color: Option<String>,
	#[serde(rename = "rangeUpperBound")]
	pub range_upper_bound: Option<f32>,
}
