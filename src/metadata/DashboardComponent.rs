use crate::metadata::ChartLegendPosition::ChartLegendPosition;
use crate::metadata::ChartRangeType::ChartRangeType;
use crate::metadata::ChartSummary::ChartSummary;
use crate::metadata::ChartTheme::ChartTheme;
use crate::metadata::ChartUnits::ChartUnits;
use crate::metadata::DashboardComponentFilter::DashboardComponentFilter;
use crate::metadata::DashboardComponentGroupingSortProperties::DashboardComponentGroupingSortProperties;
use crate::metadata::DashboardComponentType::DashboardComponentType;
use crate::metadata::DashboardDynamicValue::DashboardDynamicValue;
use crate::metadata::DashboardFilterColumn::DashboardFilterColumn;
use crate::metadata::DashboardFlexTableComponentProperties::DashboardFlexTableComponentProperties;
use crate::metadata::DashboardTableColumn::DashboardTableColumn;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DashboardComponent  {
	#[serde(rename = "autoselectColumnsFromReport")]
	pub autoselect_columns_from_report: Option<bool>,
	#[serde(rename = "chartAxisRange")]
	pub chart_axis_range: Option<ChartRangeType>,
	#[serde(rename = "chartAxisRangeMax")]
	pub chart_axis_range_max: Option<f32>,
	#[serde(rename = "chartAxisRangeMin")]
	pub chart_axis_range_min: Option<f32>,
	#[serde(rename = "chartSummary")]
	pub chart_summary: Option<Vec<ChartSummary>>,
	#[serde(rename = "componentChartTheme")]
	pub component_chart_theme: Option<ChartTheme>,
	#[serde(rename = "componentType")]
	pub component_type: DashboardComponentType,
	#[serde(rename = "dashboardDynamicValues")]
	pub dashboard_dynamic_values: Option<Vec<DashboardDynamicValue>>,
	#[serde(rename = "dashboardFilterColumns")]
	pub dashboard_filter_columns: Option<Vec<DashboardFilterColumn>>,
	#[serde(rename = "dashboardTableColumn")]
	pub dashboard_table_column: Option<Vec<DashboardTableColumn>>,
	#[serde(rename = "decimalPrecision")]
	pub decimal_precision: Option<i32>,
	#[serde(rename = "displayUnits")]
	pub display_units: Option<ChartUnits>,
	#[serde(rename = "drillDownUrl")]
	pub drill_down_url: Option<String>,
	#[serde(rename = "drillEnabled")]
	pub drill_enabled: Option<bool>,
	#[serde(rename = "drillToDetailEnabled")]
	pub drill_to_detail_enabled: Option<bool>,
	#[serde(rename = "enableHover")]
	pub enable_hover: Option<bool>,
	#[serde(rename = "expandOthers")]
	pub expand_others: Option<bool>,
	#[serde(rename = "flexComponentProperties")]
	pub flex_component_properties: Option<DashboardFlexTableComponentProperties>,
	#[serde(rename = "footer")]
	pub footer: Option<String>,
	#[serde(rename = "gaugeMax")]
	pub gauge_max: Option<f32>,
	#[serde(rename = "gaugeMin")]
	pub gauge_min: Option<f32>,
	#[serde(rename = "groupingColumn")]
	pub grouping_column: Option<Vec<String>>,
	#[serde(rename = "groupingSortProperties")]
	pub grouping_sort_properties: Option<DashboardComponentGroupingSortProperties>,
	#[serde(rename = "header")]
	pub header: Option<String>,
	#[serde(rename = "indicatorBreakpoint1")]
	pub indicator_breakpoint_1: Option<f32>,
	#[serde(rename = "indicatorBreakpoint2")]
	pub indicator_breakpoint_2: Option<f32>,
	#[serde(rename = "indicatorHighColor")]
	pub indicator_high_color: Option<String>,
	#[serde(rename = "indicatorLowColor")]
	pub indicator_low_color: Option<String>,
	#[serde(rename = "indicatorMiddleColor")]
	pub indicator_middle_color: Option<String>,
	#[serde(rename = "legendPosition")]
	pub legend_position: Option<ChartLegendPosition>,
	#[serde(rename = "maxValuesDisplayed")]
	pub max_values_displayed: Option<i32>,
	#[serde(rename = "metricLabel")]
	pub metric_label: Option<String>,
	#[serde(rename = "page")]
	pub page: Option<String>,
	#[serde(rename = "pageHeightInPixels")]
	pub page_height_in_pixels: Option<i32>,
	#[serde(rename = "report")]
	pub report: Option<String>,
	#[serde(rename = "scontrol")]
	pub scontrol: Option<String>,
	#[serde(rename = "scontrolHeightInPixels")]
	pub scontrol_height_in_pixels: Option<i32>,
	#[serde(rename = "showPercentage")]
	pub show_percentage: Option<bool>,
	#[serde(rename = "showPicturesOnCharts")]
	pub show_pictures_on_charts: Option<bool>,
	#[serde(rename = "showPicturesOnTables")]
	pub show_pictures_on_tables: Option<bool>,
	#[serde(rename = "showRange")]
	pub show_range: Option<bool>,
	#[serde(rename = "showTotal")]
	pub show_total: Option<bool>,
	#[serde(rename = "showValues")]
	pub show_values: Option<bool>,
	#[serde(rename = "sortBy")]
	pub sort_by: Option<DashboardComponentFilter>,
	#[serde(rename = "title")]
	pub title: Option<String>,
	#[serde(rename = "useReportChart")]
	pub use_report_chart: Option<bool>,
}
