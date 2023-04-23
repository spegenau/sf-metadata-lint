use crate::metadata::AudienceCriterion::AudienceCriterion;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AudienceCriteria  {
	#[serde(rename = "criterion")]
	pub criterion: Option<Vec<AudienceCriterion>>,
}
