use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RelationshipGraphDefVersion  {
	#[serde(rename = "graphDefinition")]
	pub graph_definition: String,
	#[serde(rename = "graphType")]
	pub graph_type: String,
}
