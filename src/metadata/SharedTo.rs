use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SharedTo  {
	#[serde(rename = "allCustomerPortalUsers")]
	pub all_customer_portal_users: Option<String>,
	#[serde(rename = "allInternalUsers")]
	pub all_internal_users: Option<String>,
	#[serde(rename = "allPartnerUsers")]
	pub all_partner_users: Option<String>,
	#[serde(rename = "channelProgramGroup")]
	pub channel_program_group: Option<Vec<String>>,
	#[serde(rename = "channelProgramGroups")]
	pub channel_program_groups: Option<Vec<String>>,
	#[serde(rename = "group")]
	pub group: Option<Vec<String>>,
	#[serde(rename = "groups")]
	pub groups: Option<Vec<String>>,
	#[serde(rename = "guestUser")]
	pub guest_user: Option<Vec<String>>,
	#[serde(rename = "managerSubordinates")]
	pub manager_subordinates: Option<Vec<String>>,
	#[serde(rename = "managers")]
	pub managers: Option<Vec<String>>,
	#[serde(rename = "portalRole")]
	pub portal_role: Option<Vec<String>>,
	#[serde(rename = "portalRoleAndSubordinates")]
	pub portal_role_and_subordinates: Option<Vec<String>>,
	#[serde(rename = "queue")]
	pub queue: Option<Vec<String>>,
	#[serde(rename = "role")]
	pub role: Option<Vec<String>>,
	#[serde(rename = "roleAndSubordinates")]
	pub role_and_subordinates: Option<Vec<String>>,
	#[serde(rename = "roleAndSubordinatesInternal")]
	pub role_and_subordinates_internal: Option<Vec<String>>,
	#[serde(rename = "roles")]
	pub roles: Option<Vec<String>>,
	#[serde(rename = "rolesAndSubordinates")]
	pub roles_and_subordinates: Option<Vec<String>>,
	#[serde(rename = "territories")]
	pub territories: Option<Vec<String>>,
	#[serde(rename = "territoriesAndSubordinates")]
	pub territories_and_subordinates: Option<Vec<String>>,
	#[serde(rename = "territory")]
	pub territory: Option<Vec<String>>,
	#[serde(rename = "territoryAndSubordinates")]
	pub territory_and_subordinates: Option<Vec<String>>,
}
