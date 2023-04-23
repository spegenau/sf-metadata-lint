use crate::metadata::AudienceCriteriaValue::AudienceCriteriaValue;
use crate::metadata::AudienceCriterionOperator::AudienceCriterionOperator;
use crate::metadata::AudienceCriterionType::AudienceCriterionType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AudienceCriterion  {
	#[serde(rename = "criteriaNumber")]
	pub criteria_number: Option<i32>,
	#[serde(rename = "criterionValue")]
	pub criterion_value: Option<AudienceCriteriaValue>,
	#[serde(rename = "operator")]
	pub operator: Option<AudienceCriterionOperator>,
	#[serde(rename = "type")]
	pub _type: AudienceCriterionType,
}
