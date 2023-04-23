use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AppComponentList  {
	#[serde(rename = "alignment")]
	pub alignment: String,
	#[serde(rename = "components")]
	pub components: Option<Vec<String>>,
}
