use crate::metadata::ExtendedErrorDetails::ExtendedErrorDetails;
use crate::metadata::StatusCode::StatusCode;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Error  {
	#[serde(rename = "extendedErrorDetails")]
	pub extended_error_details: Option<Vec<ExtendedErrorDetails>>,
	#[serde(rename = "fields")]
	pub fields: Option<Vec<String>>,
	#[serde(rename = "message")]
	pub message: String,
	#[serde(rename = "statusCode")]
	pub status_code: StatusCode,
}
