use crate::metadata::ID::ID;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RunTestSuccess  {
	#[serde(rename = "id")]
	pub id: ID,
	#[serde(rename = "methodName")]
	pub method_name: String,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "namespace")]
	pub namespace: String,
	#[serde(rename = "seeAllData")]
	pub see_all_data: Option<bool>,
	#[serde(rename = "time")]
	pub time: f32,
}
