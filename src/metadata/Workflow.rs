use crate::metadata::WorkflowAlert::WorkflowAlert;
use crate::metadata::WorkflowFieldUpdate::WorkflowFieldUpdate;
use crate::metadata::WorkflowFlowAction::WorkflowFlowAction;
use crate::metadata::WorkflowKnowledgePublish::WorkflowKnowledgePublish;
use crate::metadata::WorkflowOutboundMessage::WorkflowOutboundMessage;
use crate::metadata::WorkflowRule::WorkflowRule;
use crate::metadata::WorkflowSend::WorkflowSend;
use crate::metadata::WorkflowTask::WorkflowTask;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Workflow  {
	#[serde(rename = "alerts")]
	pub alerts: Option<Vec<WorkflowAlert>>,
	#[serde(rename = "fieldUpdates")]
	pub field_updates: Option<Vec<WorkflowFieldUpdate>>,
	#[serde(rename = "flowActions")]
	pub flow_actions: Option<Vec<WorkflowFlowAction>>,
	#[serde(rename = "knowledgePublishes")]
	pub knowledge_publishes: Option<Vec<WorkflowKnowledgePublish>>,
	#[serde(rename = "outboundMessages")]
	pub outbound_messages: Option<Vec<WorkflowOutboundMessage>>,
	#[serde(rename = "rules")]
	pub rules: Option<Vec<WorkflowRule>>,
	#[serde(rename = "send")]
	pub send: Option<Vec<WorkflowSend>>,
	#[serde(rename = "tasks")]
	pub tasks: Option<Vec<WorkflowTask>>,
}
