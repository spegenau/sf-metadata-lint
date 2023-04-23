use crate::metadata::BriefcaseFilterOperator::BriefcaseFilterOperator;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BriefcaseRuleFilter  {
	#[serde(rename = "filterOperator")]
	pub filter_operator: BriefcaseFilterOperator,
	#[serde(rename = "filterSeqNumber")]
	pub filter_seq_number: i32,
	#[serde(rename = "filterValue")]
	pub filter_value: Option<String>,
	#[serde(rename = "targetEntityField")]
	pub target_entity_field: String,
}
