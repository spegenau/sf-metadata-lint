use crate::metadata::UserDateInterval::UserDateInterval;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReportTimeFrameFilter  {
	#[serde(rename = "dateColumn")]
	pub date_column: String,
	#[serde(rename = "endDate")]
	pub end_date: Option<String>,
	#[serde(rename = "interval")]
	pub interval: UserDateInterval,
	#[serde(rename = "startDate")]
	pub start_date: Option<String>,
}
