use crate::metadata::ReputationPointsRule::ReputationPointsRule;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReputationPointsRules  {
	#[serde(rename = "pointsRule")]
	pub points_rule: Option<Vec<ReputationPointsRule>>,
}
