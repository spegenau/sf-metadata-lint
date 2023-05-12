use crate::metadata::ParticipantRoleAccessLevel::ParticipantRoleAccessLevel;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ParticipantRole  {
	#[serde(rename = "defaultAccessLevel")]
	pub default_access_level: ParticipantRoleAccessLevel,
	#[serde(rename = "isActive")]
	pub is_active: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "parentObject")]
	pub parent_object: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
