use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BusinessHoursEntry  {
	#[serde(rename = "active")]
	pub active: Option<bool>,
	#[serde(rename = "default")]
	pub default: bool,
	#[serde(rename = "fridayEndTime")]
	pub friday_end_time: Option<String>,
	#[serde(rename = "fridayStartTime")]
	pub friday_start_time: Option<String>,
	#[serde(rename = "mondayEndTime")]
	pub monday_end_time: Option<String>,
	#[serde(rename = "mondayStartTime")]
	pub monday_start_time: Option<String>,
	#[serde(rename = "name")]
	pub name: Option<String>,
	#[serde(rename = "saturdayEndTime")]
	pub saturday_end_time: Option<String>,
	#[serde(rename = "saturdayStartTime")]
	pub saturday_start_time: Option<String>,
	#[serde(rename = "sundayEndTime")]
	pub sunday_end_time: Option<String>,
	#[serde(rename = "sundayStartTime")]
	pub sunday_start_time: Option<String>,
	#[serde(rename = "thursdayEndTime")]
	pub thursday_end_time: Option<String>,
	#[serde(rename = "thursdayStartTime")]
	pub thursday_start_time: Option<String>,
	#[serde(rename = "timeZoneId")]
	pub time_zone_id: Option<String>,
	#[serde(rename = "tuesdayEndTime")]
	pub tuesday_end_time: Option<String>,
	#[serde(rename = "tuesdayStartTime")]
	pub tuesday_start_time: Option<String>,
	#[serde(rename = "wednesdayEndTime")]
	pub wednesday_end_time: Option<String>,
	#[serde(rename = "wednesdayStartTime")]
	pub wednesday_start_time: Option<String>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
