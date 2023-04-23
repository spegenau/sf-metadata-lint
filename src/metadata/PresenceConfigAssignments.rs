use crate::metadata::PresenceConfigProfileAssignments::PresenceConfigProfileAssignments;
use crate::metadata::PresenceConfigUserAssignments::PresenceConfigUserAssignments;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PresenceConfigAssignments  {
	#[serde(rename = "profiles")]
	pub profiles: Option<PresenceConfigProfileAssignments>,
	#[serde(rename = "users")]
	pub users: Option<PresenceConfigUserAssignments>,
}
