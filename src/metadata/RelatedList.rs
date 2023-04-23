use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RelatedList  {
	#[serde(rename = "hideOnDetail")]
	pub hide_on_detail: bool,
	#[serde(rename = "name")]
	pub name: String,
}
