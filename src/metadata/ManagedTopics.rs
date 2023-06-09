use crate::metadata::ManagedTopic::ManagedTopic;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ManagedTopics  {
	#[serde(rename = "managedTopic")]
	pub managed_topic: Option<Vec<ManagedTopic>>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
