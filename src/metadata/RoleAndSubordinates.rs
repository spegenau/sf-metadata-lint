use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RoleAndSubordinates  {
	#[serde(rename = "roleAndSubordinate")]
	pub role_and_subordinate: Option<Vec<String>>,
}
