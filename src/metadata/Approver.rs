use crate::metadata::NextOwnerType::NextOwnerType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Approver  {
	#[serde(rename = "name")]
	pub name: Option<String>,
	#[serde(rename = "type")]
	pub _type: NextOwnerType,
}
