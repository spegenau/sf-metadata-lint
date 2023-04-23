use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ObjectRelationship  {
	#[serde(rename = "join")]
	pub join: Option<Box<ObjectRelationship>>,
	#[serde(rename = "outerJoin")]
	pub outer_join: bool,
	#[serde(rename = "relationship")]
	pub relationship: String,
}
