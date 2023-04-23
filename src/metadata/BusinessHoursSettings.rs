use crate::metadata::BusinessHoursEntry::BusinessHoursEntry;
use crate::metadata::Holiday::Holiday;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BusinessHoursSettings  {
	#[serde(rename = "businessHours")]
	pub business_hours: Option<Vec<BusinessHoursEntry>>,
	#[serde(rename = "holidays")]
	pub holidays: Option<Vec<Holiday>>,
}
