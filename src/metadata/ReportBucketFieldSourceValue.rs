use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReportBucketFieldSourceValue  {
	#[serde(rename = "from")]
	pub from: Option<String>,
	#[serde(rename = "sourceValue")]
	pub source_value: Option<String>,
	#[serde(rename = "to")]
	pub to: Option<String>,
}
