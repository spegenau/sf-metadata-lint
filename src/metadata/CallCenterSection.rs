use crate::metadata::CallCenterItem::CallCenterItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CallCenterSection  {
	#[serde(rename = "items")]
	pub items: Option<Vec<CallCenterItem>>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "name")]
	pub name: String,
}
