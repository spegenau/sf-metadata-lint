use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LiveChatButtonSkills  {
	#[serde(rename = "skill")]
	pub skill: Option<Vec<String>>,
}
