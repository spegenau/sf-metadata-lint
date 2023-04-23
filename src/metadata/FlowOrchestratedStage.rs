use crate::metadata::FlowCondition::FlowCondition;
use crate::metadata::FlowConnector::FlowConnector;
use crate::metadata::FlowStageStep::FlowStageStep;
use crate::metadata::FlowStageStepExitActionInputParameter::FlowStageStepExitActionInputParameter;
use crate::metadata::FlowStageStepExitActionOutputParameter::FlowStageStepExitActionOutputParameter;
use crate::metadata::InvocableActionType::InvocableActionType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowOrchestratedStage  {
	#[serde(rename = "connector")]
	pub connector: Option<FlowConnector>,
	#[serde(rename = "exitActionInputParameters")]
	pub exit_action_input_parameters: Option<Vec<FlowStageStepExitActionInputParameter>>,
	#[serde(rename = "exitActionName")]
	pub exit_action_name: Option<String>,
	#[serde(rename = "exitActionOutputParameters")]
	pub exit_action_output_parameters: Option<Vec<FlowStageStepExitActionOutputParameter>>,
	#[serde(rename = "exitActionType")]
	pub exit_action_type: Option<InvocableActionType>,
	#[serde(rename = "exitConditionLogic")]
	pub exit_condition_logic: Option<String>,
	#[serde(rename = "exitConditions")]
	pub exit_conditions: Option<Vec<FlowCondition>>,
	#[serde(rename = "faultConnector")]
	pub fault_connector: Option<FlowConnector>,
	#[serde(rename = "stageSteps")]
	pub stage_steps: Option<Vec<FlowStageStep>>,
}
