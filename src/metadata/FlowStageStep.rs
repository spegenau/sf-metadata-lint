use crate::metadata::FlowCondition::FlowCondition;
use crate::metadata::FlowElementSubtype::FlowElementSubtype;
use crate::metadata::FlowStageStepAssignee::FlowStageStepAssignee;
use crate::metadata::FlowStageStepEntryActionInputParameter::FlowStageStepEntryActionInputParameter;
use crate::metadata::FlowStageStepEntryActionOutputParameter::FlowStageStepEntryActionOutputParameter;
use crate::metadata::FlowStageStepExitActionInputParameter::FlowStageStepExitActionInputParameter;
use crate::metadata::FlowStageStepExitActionOutputParameter::FlowStageStepExitActionOutputParameter;
use crate::metadata::FlowStageStepInputParameter::FlowStageStepInputParameter;
use crate::metadata::FlowStageStepOutputParameter::FlowStageStepOutputParameter;
use crate::metadata::InvocableActionType::InvocableActionType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowStageStep  {
	#[serde(rename = "actionName")]
	pub action_name: String,
	#[serde(rename = "actionType")]
	pub action_type: InvocableActionType,
	#[serde(rename = "assignees")]
	pub assignees: Option<Vec<FlowStageStepAssignee>>,
	#[serde(rename = "entryActionInputParameters")]
	pub entry_action_input_parameters: Option<Vec<FlowStageStepEntryActionInputParameter>>,
	#[serde(rename = "entryActionName")]
	pub entry_action_name: Option<String>,
	#[serde(rename = "entryActionOutputParameters")]
	pub entry_action_output_parameters: Option<Vec<FlowStageStepEntryActionOutputParameter>>,
	#[serde(rename = "entryActionType")]
	pub entry_action_type: Option<InvocableActionType>,
	#[serde(rename = "entryConditionLogic")]
	pub entry_condition_logic: String,
	#[serde(rename = "entryConditions")]
	pub entry_conditions: Option<Vec<FlowCondition>>,
	#[serde(rename = "exitActionInputParameters")]
	pub exit_action_input_parameters: Option<Vec<FlowStageStepExitActionInputParameter>>,
	#[serde(rename = "exitActionName")]
	pub exit_action_name: Option<String>,
	#[serde(rename = "exitActionOutputParameters")]
	pub exit_action_output_parameters: Option<Vec<FlowStageStepExitActionOutputParameter>>,
	#[serde(rename = "exitActionType")]
	pub exit_action_type: Option<InvocableActionType>,
	#[serde(rename = "inputParameters")]
	pub input_parameters: Option<Vec<FlowStageStepInputParameter>>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "outputParameters")]
	pub output_parameters: Option<Vec<FlowStageStepOutputParameter>>,
	#[serde(rename = "requiresAsyncProcessing")]
	pub requires_async_processing: Option<bool>,
	#[serde(rename = "stepSubtype")]
	pub step_subtype: Option<FlowElementSubtype>,
}
