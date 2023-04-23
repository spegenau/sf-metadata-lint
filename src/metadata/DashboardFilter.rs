use crate::metadata::DashboardFilterOption::DashboardFilterOption;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DashboardFilter  {
	#[serde(rename = "dashboardFilterOptions")]
	pub dashboard_filter_options: Option<Vec<DashboardFilterOption>>,
	#[serde(rename = "name")]
	pub name: String,
}
