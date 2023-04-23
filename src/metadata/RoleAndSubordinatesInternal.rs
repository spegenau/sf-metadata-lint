use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RoleAndSubordinatesInternal  {
	#[serde(rename = "roleAndSubordinateInternal")]
	pub role_and_subordinate_internal: Option<Vec<String>>,
}
