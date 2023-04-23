use crate::metadata::CustomLabel::CustomLabel;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomLabels  {
	#[serde(rename = "labels")]
	pub labels: Option<Vec<CustomLabel>>,
}
