use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ProfileLoginHours  {
	#[serde(rename = "fridayEnd")]
	pub friday_end: Option<String>,
	#[serde(rename = "fridayStart")]
	pub friday_start: Option<String>,
	#[serde(rename = "mondayEnd")]
	pub monday_end: Option<String>,
	#[serde(rename = "mondayStart")]
	pub monday_start: Option<String>,
	#[serde(rename = "saturdayEnd")]
	pub saturday_end: Option<String>,
	#[serde(rename = "saturdayStart")]
	pub saturday_start: Option<String>,
	#[serde(rename = "sundayEnd")]
	pub sunday_end: Option<String>,
	#[serde(rename = "sundayStart")]
	pub sunday_start: Option<String>,
	#[serde(rename = "thursdayEnd")]
	pub thursday_end: Option<String>,
	#[serde(rename = "thursdayStart")]
	pub thursday_start: Option<String>,
	#[serde(rename = "tuesdayEnd")]
	pub tuesday_end: Option<String>,
	#[serde(rename = "tuesdayStart")]
	pub tuesday_start: Option<String>,
	#[serde(rename = "wednesdayEnd")]
	pub wednesday_end: Option<String>,
	#[serde(rename = "wednesdayStart")]
	pub wednesday_start: Option<String>,
}
