use crate::metadata::ReputationLevel::ReputationLevel;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReputationLevelDefinitions  {
	#[serde(rename = "level")]
	pub level: Option<Vec<ReputationLevel>>,
}
