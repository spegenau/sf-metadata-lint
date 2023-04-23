use crate::metadata::AsyncRequestState::AsyncRequestState;
use crate::metadata::ID::ID;
use crate::metadata::StatusCode::StatusCode;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AsyncResult  {
	#[serde(rename = "done")]
	pub done: bool,
	#[serde(rename = "id")]
	pub id: ID,
	#[serde(rename = "message")]
	pub message: Option<String>,
	#[serde(rename = "state")]
	pub state: AsyncRequestState,
	#[serde(rename = "statusCode")]
	pub status_code: Option<StatusCode>,
}
