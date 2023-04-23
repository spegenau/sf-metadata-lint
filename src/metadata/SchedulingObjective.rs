use crate::metadata::SchedulingCategory::SchedulingCategory;
use crate::metadata::SchedulingObjectiveParameter::SchedulingObjectiveParameter;
use crate::metadata::SchedulingObjectiveType::SchedulingObjectiveType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SchedulingObjective  {
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "schedulingCategory")]
	pub scheduling_category: SchedulingCategory,
	#[serde(rename = "schedulingObjectiveParameters")]
	pub scheduling_objective_parameters: Option<Vec<SchedulingObjectiveParameter>>,
	#[serde(rename = "schedulingObjectiveType")]
	pub scheduling_objective_type: SchedulingObjectiveType,
}
