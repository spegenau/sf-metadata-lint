use crate::metadata::DashboardFilterOperation::DashboardFilterOperation;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DashboardFilterOption  {
	#[serde(rename = "operator")]
	pub operator: DashboardFilterOperation,
	#[serde(rename = "values")]
	pub values: Option<Vec<String>>,
}
