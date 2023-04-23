use crate::metadata::PipelineInspectionMetric::PipelineInspectionMetric;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PipelineInspMetricConfig  {
	#[serde(rename = "isCumulative")]
	pub is_cumulative: bool,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "metric")]
	pub metric: PipelineInspectionMetric,
}
