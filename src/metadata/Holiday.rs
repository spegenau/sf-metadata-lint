use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Holiday  {
	#[serde(rename = "activityDate")]
	pub activity_date: Option<String>,
	#[serde(rename = "businessHours")]
	pub business_hours: Option<Vec<String>>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "endTime")]
	pub end_time: Option<String>,
	#[serde(rename = "isRecurring")]
	pub is_recurring: Option<bool>,
	#[serde(rename = "name")]
	pub name: Option<String>,
	#[serde(rename = "recurrenceDayOfMonth")]
	pub recurrence_day_of_month: Option<i32>,
	#[serde(rename = "recurrenceDayOfWeek")]
	pub recurrence_day_of_week: Option<Vec<String>>,
	#[serde(rename = "recurrenceDayOfWeekMask")]
	pub recurrence_day_of_week_mask: Option<i32>,
	#[serde(rename = "recurrenceEndDate")]
	pub recurrence_end_date: Option<String>,
	#[serde(rename = "recurrenceInstance")]
	pub recurrence_instance: Option<String>,
	#[serde(rename = "recurrenceInterval")]
	pub recurrence_interval: Option<i32>,
	#[serde(rename = "recurrenceMonthOfYear")]
	pub recurrence_month_of_year: Option<String>,
	#[serde(rename = "recurrenceStartDate")]
	pub recurrence_start_date: Option<String>,
	#[serde(rename = "recurrenceType")]
	pub recurrence_type: Option<String>,
	#[serde(rename = "startTime")]
	pub start_time: Option<String>,
}
