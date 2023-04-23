use crate::metadata::ReportBucketFieldType::ReportBucketFieldType;
use crate::metadata::ReportBucketFieldValue::ReportBucketFieldValue;
use crate::metadata::ReportFormulaNullTreatment::ReportFormulaNullTreatment;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReportBucketField  {
	#[serde(rename = "bucketType")]
	pub bucket_type: ReportBucketFieldType,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "nullTreatment")]
	pub null_treatment: Option<ReportFormulaNullTreatment>,
	#[serde(rename = "otherBucketLabel")]
	pub other_bucket_label: Option<String>,
	#[serde(rename = "sourceColumnName")]
	pub source_column_name: String,
	#[serde(rename = "useOther")]
	pub use_other: Option<bool>,
	#[serde(rename = "values")]
	pub values: Option<Vec<ReportBucketFieldValue>>,
}
