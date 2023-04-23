use crate::metadata::DashboardComponentColumn::DashboardComponentColumn;
use crate::metadata::DashboardComponentSortInfo::DashboardComponentSortInfo;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DashboardFlexTableComponentProperties  {
	#[serde(rename = "decimalPrecision")]
	pub decimal_precision: Option<i32>,
	#[serde(rename = "flexTableColumn")]
	pub flex_table_column: Option<Vec<DashboardComponentColumn>>,
	#[serde(rename = "flexTableSortInfo")]
	pub flex_table_sort_info: Option<DashboardComponentSortInfo>,
	#[serde(rename = "hideChatterPhotos")]
	pub hide_chatter_photos: Option<bool>,
}
