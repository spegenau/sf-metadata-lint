use crate::metadata::PublicGroups::PublicGroups;
use crate::metadata::RoleAndSubordinates::RoleAndSubordinates;
use crate::metadata::RoleAndSubordinatesInternal::RoleAndSubordinatesInternal;
use crate::metadata::Roles::Roles;
use crate::metadata::Users::Users;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct QueueMembers  {
	#[serde(rename = "publicGroups")]
	pub public_groups: Option<PublicGroups>,
	#[serde(rename = "roleAndSubordinates")]
	pub role_and_subordinates: Option<RoleAndSubordinates>,
	#[serde(rename = "roleAndSubordinatesInternal")]
	pub role_and_subordinates_internal: Option<RoleAndSubordinatesInternal>,
	#[serde(rename = "roles")]
	pub roles: Option<Roles>,
	#[serde(rename = "users")]
	pub users: Option<Users>,
}
