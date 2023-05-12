use crate::metadata::AudienceCriteria::AudienceCriteria;
use crate::metadata::FormulaFilterType::FormulaFilterType;
use crate::metadata::PersonalizationTargetInfos::PersonalizationTargetInfos;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Audience  {
	#[serde(rename = "audienceName")]
	pub audience_name: String,
	#[serde(rename = "container")]
	pub container: String,
	#[serde(rename = "criteria")]
	pub criteria: AudienceCriteria,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "formula")]
	pub formula: Option<String>,
	#[serde(rename = "formulaFilterType")]
	pub formula_filter_type: Option<FormulaFilterType>,
	#[serde(rename = "isDefaultAudience")]
	pub is_default_audience: Option<bool>,
	#[serde(rename = "targets")]
	pub targets: Option<PersonalizationTargetInfos>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
