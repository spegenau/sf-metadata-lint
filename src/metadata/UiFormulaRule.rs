use crate::metadata::UiFormulaCriterion::UiFormulaCriterion;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct UiFormulaRule  {
	#[serde(rename = "booleanFilter")]
	pub boolean_filter: Option<String>,
	#[serde(rename = "criteria")]
	pub criteria: Option<Vec<UiFormulaCriterion>>,
}
