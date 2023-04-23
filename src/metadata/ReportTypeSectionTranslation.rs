use crate::metadata::ReportTypeColumnTranslation::ReportTypeColumnTranslation;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReportTypeSectionTranslation  {
	#[serde(rename = "columns")]
	pub columns: Option<Vec<ReportTypeColumnTranslation>>,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "name")]
	pub name: String,
}
