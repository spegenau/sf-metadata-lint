use crate::metadata::DashboardComponent::DashboardComponent;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DashboardGridComponent  {
	#[serde(rename = "colSpan")]
	pub col_span: i32,
	#[serde(rename = "columnIndex")]
	pub column_index: i32,
	#[serde(rename = "dashboardComponent")]
	pub dashboard_component: DashboardComponent,
	#[serde(rename = "rowIndex")]
	pub row_index: i32,
	#[serde(rename = "rowSpan")]
	pub row_span: i32,
}
