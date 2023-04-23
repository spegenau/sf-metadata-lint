use crate::metadata::DaysOfWeek::DaysOfWeek;
use crate::metadata::TimeSheetFrequency::TimeSheetFrequency;
use crate::metadata::TimeSheetTemplateAssignment::TimeSheetTemplateAssignment;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct TimeSheetTemplate  {
	#[serde(rename = "active")]
	pub active: bool,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "frequency")]
	pub frequency: TimeSheetFrequency,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "startDate")]
	pub start_date: String,
	#[serde(rename = "timeSheetTemplateAssignments")]
	pub time_sheet_template_assignments: Option<Vec<TimeSheetTemplateAssignment>>,
	#[serde(rename = "workWeekEndDay")]
	pub work_week_end_day: DaysOfWeek,
	#[serde(rename = "workWeekStartDay")]
	pub work_week_start_day: DaysOfWeek,
}
