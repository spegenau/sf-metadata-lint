use crate::metadata::ReportTypeSectionTranslation::ReportTypeSectionTranslation;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReportTypeTranslation  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "sections")]
	pub sections: Option<Vec<ReportTypeSectionTranslation>>,
}
