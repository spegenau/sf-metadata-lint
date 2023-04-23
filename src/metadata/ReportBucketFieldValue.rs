use crate::metadata::ReportBucketFieldSourceValue::ReportBucketFieldSourceValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReportBucketFieldValue  {
	#[serde(rename = "sourceValues")]
	pub source_values: Option<Vec<ReportBucketFieldSourceValue>>,
	#[serde(rename = "value")]
	pub value: String,
}
