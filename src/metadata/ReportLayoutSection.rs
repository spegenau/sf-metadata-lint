use crate::metadata::ReportTypeColumn::ReportTypeColumn;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReportLayoutSection  {
	#[serde(rename = "columns")]
	pub columns: Option<Vec<ReportTypeColumn>>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
}
