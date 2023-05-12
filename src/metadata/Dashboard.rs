use crate::metadata::ChartBackgroundDirection::ChartBackgroundDirection;
use crate::metadata::ChartColorPalettes::ChartColorPalettes;
use crate::metadata::ChartTheme::ChartTheme;
use crate::metadata::DashboardComponentSection::DashboardComponentSection;
use crate::metadata::DashboardFilter::DashboardFilter;
use crate::metadata::DashboardGridLayout::DashboardGridLayout;
use crate::metadata::DashboardType::DashboardType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Dashboard  {
	#[serde(rename = "backgroundEndColor")]
	pub background_end_color: String,
	#[serde(rename = "backgroundFadeDirection")]
	pub background_fade_direction: ChartBackgroundDirection,
	#[serde(rename = "backgroundStartColor")]
	pub background_start_color: String,
	#[serde(rename = "chartTheme")]
	pub chart_theme: Option<ChartTheme>,
	#[serde(rename = "colorPalette")]
	pub color_palette: Option<ChartColorPalettes>,
	#[serde(rename = "dashboardChartTheme")]
	pub dashboard_chart_theme: Option<ChartTheme>,
	#[serde(rename = "dashboardColorPalette")]
	pub dashboard_color_palette: Option<ChartColorPalettes>,
	#[serde(rename = "dashboardFilters")]
	pub dashboard_filters: Option<Vec<DashboardFilter>>,
	#[serde(rename = "dashboardGridLayout")]
	pub dashboard_grid_layout: Option<DashboardGridLayout>,
	#[serde(rename = "dashboardResultRefreshedDate")]
	pub dashboard_result_refreshed_date: Option<String>,
	#[serde(rename = "dashboardResultRunningUser")]
	pub dashboard_result_running_user: Option<String>,
	#[serde(rename = "dashboardType")]
	pub dashboard_type: Option<DashboardType>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "folderName")]
	pub folder_name: Option<String>,
	#[serde(rename = "isGridLayout")]
	pub is_grid_layout: Option<bool>,
	#[serde(rename = "leftSection")]
	pub left_section: Option<DashboardComponentSection>,
	#[serde(rename = "middleSection")]
	pub middle_section: Option<DashboardComponentSection>,
	#[serde(rename = "numSubscriptions")]
	pub num_subscriptions: Option<i32>,
	#[serde(rename = "rightSection")]
	pub right_section: Option<DashboardComponentSection>,
	#[serde(rename = "runningUser")]
	pub running_user: Option<String>,
	#[serde(rename = "textColor")]
	pub text_color: String,
	#[serde(rename = "title")]
	pub title: String,
	#[serde(rename = "titleColor")]
	pub title_color: String,
	#[serde(rename = "titleSize")]
	pub title_size: i32,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
