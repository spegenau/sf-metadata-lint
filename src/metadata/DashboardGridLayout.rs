use crate::metadata::DashboardGridComponent::DashboardGridComponent;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DashboardGridLayout  {
	#[serde(rename = "dashboardGridComponents")]
	pub dashboard_grid_components: Option<Vec<DashboardGridComponent>>,
	#[serde(rename = "numberOfColumns")]
	pub number_of_columns: i32,
	#[serde(rename = "rowHeight")]
	pub row_height: i32,
}
