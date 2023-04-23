use crate::metadata::PersonalizationTargetInfo::PersonalizationTargetInfo;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PersonalizationTargetInfos  {
	#[serde(rename = "target")]
	pub target: Option<Vec<PersonalizationTargetInfo>>,
}
