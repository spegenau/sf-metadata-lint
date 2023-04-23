use crate::metadata::ID::ID;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CodeCoverageWarning  {
	#[serde(rename = "id")]
	pub id: ID,
	#[serde(rename = "message")]
	pub message: String,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "namespace")]
	pub namespace: String,
}
