use crate::metadata::OauthCustomScopeApp::OauthCustomScopeApp;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OauthCustomScope  {
	#[serde(rename = "assignedTo")]
	pub assigned_to: Option<Vec<OauthCustomScopeApp>>,
	#[serde(rename = "description")]
	pub description: String,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "isPublic")]
	pub is_public: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
}
