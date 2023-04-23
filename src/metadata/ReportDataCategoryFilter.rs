use crate::metadata::DataCategoryFilterOperation::DataCategoryFilterOperation;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReportDataCategoryFilter  {
	#[serde(rename = "dataCategory")]
	pub data_category: String,
	#[serde(rename = "dataCategoryGroup")]
	pub data_category_group: String,
	#[serde(rename = "operator")]
	pub operator: DataCategoryFilterOperation,
}
