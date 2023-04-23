use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct KnowledgeWorkOrderLineItemField  {
	#[serde(rename = "name")]
	pub name: Option<String>,
}
