use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OrgSettings  {
	#[serde(rename = "enableCustomerSuccessPortal")]
	pub enable_customer_success_portal: Option<bool>,
	#[serde(rename = "enableIncludeContractStatus")]
	pub enable_include_contract_status: Option<bool>,
	#[serde(rename = "enableMakeDeploymentsMandatory")]
	pub enable_make_deployments_mandatory: Option<bool>,
	#[serde(rename = "enableManageSelfServiceUsers")]
	pub enable_manage_self_service_users: Option<bool>,
	#[serde(rename = "enableOrgFeedSentimentAnalysis")]
	pub enable_org_feed_sentiment_analysis: Option<bool>,
	#[serde(rename = "enableRADeploymentAttributeOnly")]
	pub enable_ra_deployment_attribute_only: Option<bool>,
	#[serde(rename = "enableResetDivisionOnLogin")]
	pub enable_reset_division_on_login: Option<bool>,
}
