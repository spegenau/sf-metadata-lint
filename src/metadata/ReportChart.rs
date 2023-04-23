use crate::metadata::ChartBackgroundDirection::ChartBackgroundDirection;
use crate::metadata::ChartLegendPosition::ChartLegendPosition;
use crate::metadata::ChartPosition::ChartPosition;
use crate::metadata::ChartRangeType::ChartRangeType;
use crate::metadata::ChartSummary::ChartSummary;
use crate::metadata::ChartType::ChartType;
use crate::metadata::ReportChartSize::ReportChartSize;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReportChart  {
	#[serde(rename = "backgroundColor1")]
	pub background_color_1: Option<String>,
	#[serde(rename = "backgroundColor2")]
	pub background_color_2: Option<String>,
	#[serde(rename = "backgroundFadeDir")]
	pub background_fade_dir: Option<ChartBackgroundDirection>,
	#[serde(rename = "chartSummaries")]
	pub chart_summaries: Option<Vec<ChartSummary>>,
	#[serde(rename = "chartType")]
	pub chart_type: ChartType,
	#[serde(rename = "enableHoverLabels")]
	pub enable_hover_labels: Option<bool>,
	#[serde(rename = "expandOthers")]
	pub expand_others: Option<bool>,
	#[serde(rename = "groupingColumn")]
	pub grouping_column: Option<String>,
	#[serde(rename = "legendPosition")]
	pub legend_position: Option<ChartLegendPosition>,
	#[serde(rename = "location")]
	pub location: Option<ChartPosition>,
	#[serde(rename = "secondaryGroupingColumn")]
	pub secondary_grouping_column: Option<String>,
	#[serde(rename = "showAxisLabels")]
	pub show_axis_labels: Option<bool>,
	#[serde(rename = "showPercentage")]
	pub show_percentage: Option<bool>,
	#[serde(rename = "showTotal")]
	pub show_total: Option<bool>,
	#[serde(rename = "showValues")]
	pub show_values: Option<bool>,
	#[serde(rename = "size")]
	pub size: Option<ReportChartSize>,
	#[serde(rename = "summaryAxisManualRangeEnd")]
	pub summary_axis_manual_range_end: Option<f32>,
	#[serde(rename = "summaryAxisManualRangeStart")]
	pub summary_axis_manual_range_start: Option<f32>,
	#[serde(rename = "summaryAxisRange")]
	pub summary_axis_range: Option<ChartRangeType>,
	#[serde(rename = "textColor")]
	pub text_color: Option<String>,
	#[serde(rename = "textSize")]
	pub text_size: Option<i32>,
	#[serde(rename = "title")]
	pub title: Option<String>,
	#[serde(rename = "titleColor")]
	pub title_color: Option<String>,
	#[serde(rename = "titleSize")]
	pub title_size: Option<i32>,
}
