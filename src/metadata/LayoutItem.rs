use crate::metadata::AnalyticsCloudComponentLayoutItem::AnalyticsCloudComponentLayoutItem;
use crate::metadata::ReportChartComponentLayoutItem::ReportChartComponentLayoutItem;
use crate::metadata::UiBehavior::UiBehavior;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LayoutItem  {
	#[serde(rename = "analyticsCloudComponent")]
	pub analytics_cloud_component: Option<AnalyticsCloudComponentLayoutItem>,
	#[serde(rename = "behavior")]
	pub behavior: Option<UiBehavior>,
	#[serde(rename = "canvas")]
	pub canvas: Option<String>,
	#[serde(rename = "component")]
	pub component: Option<String>,
	#[serde(rename = "customLink")]
	pub custom_link: Option<String>,
	#[serde(rename = "emptySpace")]
	pub empty_space: Option<bool>,
	#[serde(rename = "field")]
	pub field: Option<String>,
	#[serde(rename = "height")]
	pub height: Option<i32>,
	#[serde(rename = "page")]
	pub page: Option<String>,
	#[serde(rename = "reportChartComponent")]
	pub report_chart_component: Option<ReportChartComponentLayoutItem>,
	#[serde(rename = "scontrol")]
	pub scontrol: Option<String>,
	#[serde(rename = "showLabel")]
	pub show_label: Option<bool>,
	#[serde(rename = "showScrollbars")]
	pub show_scrollbars: Option<bool>,
	#[serde(rename = "width")]
	pub width: Option<String>,
}
