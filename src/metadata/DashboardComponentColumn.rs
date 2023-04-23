use crate::metadata::DashboardComponentColumnType::DashboardComponentColumnType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DashboardComponentColumn  {
	#[serde(rename = "breakPoint1")]
	pub break_point_1: Option<f32>,
	#[serde(rename = "breakPoint2")]
	pub break_point_2: Option<f32>,
	#[serde(rename = "breakPointOrder")]
	pub break_point_order: Option<i32>,
	#[serde(rename = "highRangeColor")]
	pub high_range_color: Option<i32>,
	#[serde(rename = "lowRangeColor")]
	pub low_range_color: Option<i32>,
	#[serde(rename = "midRangeColor")]
	pub mid_range_color: Option<i32>,
	#[serde(rename = "reportColumn")]
	pub report_column: String,
	#[serde(rename = "showSubTotal")]
	pub show_sub_total: Option<bool>,
	#[serde(rename = "showTotal")]
	pub show_total: Option<bool>,
	#[serde(rename = "type")]
	pub _type: DashboardComponentColumnType,
}
