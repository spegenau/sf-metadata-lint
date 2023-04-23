use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct NetworkMemberGroup  {
	#[serde(rename = "permissionSet")]
	pub permission_set: Option<Vec<String>>,
	#[serde(rename = "profile")]
	pub profile: Option<Vec<String>>,
}
