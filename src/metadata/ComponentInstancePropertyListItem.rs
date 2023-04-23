use crate::metadata::UiFormulaRule::UiFormulaRule;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ComponentInstancePropertyListItem  {
	#[serde(rename = "value")]
	pub value: Option<String>,
	#[serde(rename = "visibilityRule")]
	pub visibility_rule: Option<UiFormulaRule>,
}
