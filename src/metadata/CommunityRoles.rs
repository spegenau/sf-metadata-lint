use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CommunityRoles  {
	#[serde(rename = "customerUserRole")]
	pub customer_user_role: Option<String>,
	#[serde(rename = "employeeUserRole")]
	pub employee_user_role: Option<String>,
	#[serde(rename = "partnerUserRole")]
	pub partner_user_role: Option<String>,
}
