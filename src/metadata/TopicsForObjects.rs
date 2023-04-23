use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct TopicsForObjects  {
	#[serde(rename = "enableTopics")]
	pub enable_topics: bool,
	#[serde(rename = "entityApiName")]
	pub entity_api_name: String,
}
