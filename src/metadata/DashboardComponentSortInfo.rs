use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DashboardComponentSortInfo  {
	#[serde(rename = "sortColumn")]
	pub sort_column: Option<String>,
	#[serde(rename = "sortOrder")]
	pub sort_order: Option<String>,
}
