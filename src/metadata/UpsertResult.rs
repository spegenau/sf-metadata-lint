use crate::metadata::Error::Error;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct UpsertResult  {
	#[serde(rename = "created")]
	pub created: bool,
	#[serde(rename = "errors")]
	pub errors: Option<Vec<Error>>,
	#[serde(rename = "fullName")]
	pub full_name: String,
	#[serde(rename = "success")]
	pub success: bool,
}
