use crate::metadata::ID::ID;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CancelDeployResult  {
	#[serde(rename = "done")]
	pub done: bool,
	#[serde(rename = "id")]
	pub id: ID,
}
