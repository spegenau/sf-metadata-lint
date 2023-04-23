use crate::metadata::DashboardComponentGroupingSort::DashboardComponentGroupingSort;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DashboardComponentGroupingSortProperties  {
	#[serde(rename = "groupingSorts")]
	pub grouping_sorts: Option<Vec<DashboardComponentGroupingSort>>,
}
