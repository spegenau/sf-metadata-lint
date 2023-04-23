use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct KnowledgeCommunitiesSettings  {
	#[serde(rename = "community")]
	pub community: Option<Vec<String>>,
}
