use crate::metadata::PeriodTypes::PeriodTypes;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ForecastRangeSettings  {
	#[serde(rename = "beginning")]
	pub beginning: i32,
	#[serde(rename = "displaying")]
	pub displaying: i32,
	#[serde(rename = "periodType")]
	pub period_type: PeriodTypes,
}
