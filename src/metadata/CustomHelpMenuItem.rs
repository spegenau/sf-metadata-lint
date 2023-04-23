use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomHelpMenuItem  {
	#[serde(rename = "linkUrl")]
	pub link_url: String,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "sortOrder")]
	pub sort_order: i32,
}
