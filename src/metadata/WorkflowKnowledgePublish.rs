use crate::metadata::KnowledgeWorkflowAction::KnowledgeWorkflowAction;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WorkflowKnowledgePublish  {
	#[serde(rename = "action")]
	pub action: KnowledgeWorkflowAction,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "language")]
	pub language: Option<String>,
	#[serde(rename = "protected")]
	pub protected: bool,
}
