use crate::metadata::CareLimitTypeMetricType::CareLimitTypeMetricType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CareLimitType  {
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "limitType")]
	pub limit_type: Option<String>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "metricType")]
	pub metric_type: Option<CareLimitTypeMetricType>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
