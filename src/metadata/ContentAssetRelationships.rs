use crate::metadata::ContentAssetLink::ContentAssetLink;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ContentAssetRelationships  {
	#[serde(rename = "emailTemplate")]
	pub email_template: Option<Vec<ContentAssetLink>>,
	#[serde(rename = "insightsApplication")]
	pub insights_application: Option<Vec<ContentAssetLink>>,
	#[serde(rename = "network")]
	pub network: Option<Vec<ContentAssetLink>>,
	#[serde(rename = "organization")]
	pub organization: Option<ContentAssetLink>,
	#[serde(rename = "workspace")]
	pub workspace: Option<Vec<ContentAssetLink>>,
}
