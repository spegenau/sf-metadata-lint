use crate::metadata::ID::ID;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RunTestFailure  {
	#[serde(rename = "id")]
	pub id: ID,
	#[serde(rename = "message")]
	pub message: String,
	#[serde(rename = "methodName")]
	pub method_name: String,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "namespace")]
	pub namespace: String,
	#[serde(rename = "packageName")]
	pub package_name: String,
	#[serde(rename = "seeAllData")]
	pub see_all_data: Option<bool>,
	#[serde(rename = "stackTrace")]
	pub stack_trace: String,
	#[serde(rename = "time")]
	pub time: f32,
	#[serde(rename = "type")]
	pub _type: String,
}
