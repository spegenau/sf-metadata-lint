use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CommandActionResponse  {
	#[serde(rename = "template")]
	pub template: String,
}
