use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ManagedTopic  {
	#[serde(rename = "managedTopicType")]
	pub managed_topic_type: String,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "parentName")]
	pub parent_name: String,
	#[serde(rename = "position")]
	pub position: i32,
	#[serde(rename = "topicDescription")]
	pub topic_description: String,
}
