use crate::metadata::MilestoneTypeRecurrenceType::MilestoneTypeRecurrenceType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MilestoneType  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "recurrenceType")]
	pub recurrence_type: Option<MilestoneTypeRecurrenceType>,
}
