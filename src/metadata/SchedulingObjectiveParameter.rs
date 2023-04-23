use crate::metadata::ObjectiveParameterKey::ObjectiveParameterKey;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SchedulingObjectiveParameter  {
	#[serde(rename = "parameterKey")]
	pub parameter_key: ObjectiveParameterKey,
	#[serde(rename = "value")]
	pub value: Option<String>,
}
