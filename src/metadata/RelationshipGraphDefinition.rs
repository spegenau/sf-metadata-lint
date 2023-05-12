use crate::metadata::RelationshipGraphDefVersion::RelationshipGraphDefVersion;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RelationshipGraphDefinition  {
	#[serde(rename = "isActive")]
	pub is_active: bool,
	#[serde(rename = "isTemplate")]
	pub is_template: bool,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "relationshipGraphDefVersions")]
	pub relationship_graph_def_versions: Option<Vec<RelationshipGraphDefVersion>>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
