use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CampaignInfluenceModel  {
	#[serde(rename = "isActive")]
	pub is_active: Option<bool>,
	#[serde(rename = "isDefaultModel")]
	pub is_default_model: bool,
	#[serde(rename = "isModelLocked")]
	pub is_model_locked: bool,
	#[serde(rename = "modelDescription")]
	pub model_description: Option<String>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "recordPreference")]
	pub record_preference: Option<String>,
}
