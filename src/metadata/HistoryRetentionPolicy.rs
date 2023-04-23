use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct HistoryRetentionPolicy  {
	#[serde(rename = "archiveAfterMonths")]
	pub archive_after_months: i32,
	#[serde(rename = "archiveRetentionYears")]
	pub archive_retention_years: i32,
	#[serde(rename = "description")]
	pub description: Option<String>,
}
