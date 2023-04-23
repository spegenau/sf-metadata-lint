use crate::metadata::FileProperties::FileProperties;
use crate::metadata::RetrieveMessage::RetrieveMessage;
use crate::metadata::RetrieveStatus::RetrieveStatus;
use crate::metadata::StatusCode::StatusCode;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RetrieveResult  {
	#[serde(rename = "done")]
	pub done: bool,
	#[serde(rename = "errorMessage")]
	pub error_message: Option<String>,
	#[serde(rename = "errorStatusCode")]
	pub error_status_code: Option<StatusCode>,
	#[serde(rename = "fileProperties")]
	pub file_properties: Option<Vec<FileProperties>>,
	#[serde(rename = "id")]
	pub id: String,
	#[serde(rename = "messages")]
	pub messages: Option<Vec<RetrieveMessage>>,
	#[serde(rename = "status")]
	pub status: RetrieveStatus,
	#[serde(rename = "success")]
	pub success: bool,
	#[serde(rename = "zipFile")]
	pub zip_file: String,
}
