use crate::metadata::AnalyticSnapshotMapping::AnalyticSnapshotMapping;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AnalyticSnapshot  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "groupColumn")]
	pub group_column: Option<String>,
	#[serde(rename = "mappings")]
	pub mappings: Option<Vec<AnalyticSnapshotMapping>>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "runningUser")]
	pub running_user: Option<String>,
	#[serde(rename = "sourceReport")]
	pub source_report: String,
	#[serde(rename = "targetObject")]
	pub target_object: String,
}
