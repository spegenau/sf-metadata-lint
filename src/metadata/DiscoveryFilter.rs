use crate::metadata::DiscoveryFilterFieldType::DiscoveryFilterFieldType;
use crate::metadata::DiscoveryFilterOperator::DiscoveryFilterOperator;
use crate::metadata::DiscoveryFilterValue::DiscoveryFilterValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DiscoveryFilter  {
	#[serde(rename = "field")]
	pub field: String,
	#[serde(rename = "operator")]
	pub operator: DiscoveryFilterOperator,
	#[serde(rename = "type")]
	pub _type: Option<DiscoveryFilterFieldType>,
	#[serde(rename = "values")]
	pub values: Option<Vec<DiscoveryFilterValue>>,
}
