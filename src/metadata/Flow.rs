use crate::metadata::FlowActionCall::FlowActionCall;
use crate::metadata::FlowApexPluginCall::FlowApexPluginCall;
use crate::metadata::FlowAssignment::FlowAssignment;
use crate::metadata::FlowChoice::FlowChoice;
use crate::metadata::FlowCollectionProcessor::FlowCollectionProcessor;
use crate::metadata::FlowConstant::FlowConstant;
use crate::metadata::FlowDecision::FlowDecision;
use crate::metadata::FlowDynamicChoiceSet::FlowDynamicChoiceSet;
use crate::metadata::FlowEnvironment::FlowEnvironment;
use crate::metadata::FlowFormula::FlowFormula;
use crate::metadata::FlowLoop::FlowLoop;
use crate::metadata::FlowMetadataValue::FlowMetadataValue;
use crate::metadata::FlowOrchestratedStage::FlowOrchestratedStage;
use crate::metadata::FlowProcessType::FlowProcessType;
use crate::metadata::FlowRecordCreate::FlowRecordCreate;
use crate::metadata::FlowRecordDelete::FlowRecordDelete;
use crate::metadata::FlowRecordLookup::FlowRecordLookup;
use crate::metadata::FlowRecordRollback::FlowRecordRollback;
use crate::metadata::FlowRecordUpdate::FlowRecordUpdate;
use crate::metadata::FlowRunInMode::FlowRunInMode;
use crate::metadata::FlowScreen::FlowScreen;
use crate::metadata::FlowStage::FlowStage;
use crate::metadata::FlowStart::FlowStart;
use crate::metadata::FlowStep::FlowStep;
use crate::metadata::FlowSubflow::FlowSubflow;
use crate::metadata::FlowTextTemplate::FlowTextTemplate;
use crate::metadata::FlowTransform::FlowTransform;
use crate::metadata::FlowVariable::FlowVariable;
use crate::metadata::FlowVersionStatus::FlowVersionStatus;
use crate::metadata::FlowWait::FlowWait;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Flow  {
	#[serde(rename = "actionCalls")]
	pub action_calls: Option<Vec<FlowActionCall>>,
	#[serde(rename = "apexPluginCalls")]
	pub apex_plugin_calls: Option<Vec<FlowApexPluginCall>>,
	#[serde(rename = "apiVersion")]
	pub api_version: Option<f32>,
	#[serde(rename = "assignments")]
	pub assignments: Option<Vec<FlowAssignment>>,
	#[serde(rename = "associatedRecord")]
	pub associated_record: Option<String>,
	#[serde(rename = "choices")]
	pub choices: Option<Vec<FlowChoice>>,
	#[serde(rename = "collectionProcessors")]
	pub collection_processors: Option<Vec<FlowCollectionProcessor>>,
	#[serde(rename = "constants")]
	pub constants: Option<Vec<FlowConstant>>,
	#[serde(rename = "decisions")]
	pub decisions: Option<Vec<FlowDecision>>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "dynamicChoiceSets")]
	pub dynamic_choice_sets: Option<Vec<FlowDynamicChoiceSet>>,
	#[serde(rename = "environments")]
	pub environments: Option<Vec<FlowEnvironment>>,
	#[serde(rename = "formulas")]
	pub formulas: Option<Vec<FlowFormula>>,
	#[serde(rename = "interviewLabel")]
	pub interview_label: Option<String>,
	#[serde(rename = "isAdditionalPermissionRequiredToRun")]
	pub is_additional_permission_required_to_run: Option<bool>,
	#[serde(rename = "isOverridable")]
	pub is_overridable: Option<bool>,
	#[serde(rename = "isTemplate")]
	pub is_template: Option<bool>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "loops")]
	pub loops: Option<Vec<FlowLoop>>,
	#[serde(rename = "migratedFromWorkflowRuleName")]
	pub migrated_from_workflow_rule_name: Option<String>,
	#[serde(rename = "orchestratedStages")]
	pub orchestrated_stages: Option<Vec<FlowOrchestratedStage>>,
	#[serde(rename = "overriddenFlow")]
	pub overridden_flow: Option<String>,
	#[serde(rename = "processMetadataValues")]
	pub process_metadata_values: Option<Vec<FlowMetadataValue>>,
	#[serde(rename = "processType")]
	pub process_type: Option<FlowProcessType>,
	#[serde(rename = "recordCreates")]
	pub record_creates: Option<Vec<FlowRecordCreate>>,
	#[serde(rename = "recordDeletes")]
	pub record_deletes: Option<Vec<FlowRecordDelete>>,
	#[serde(rename = "recordLookups")]
	pub record_lookups: Option<Vec<FlowRecordLookup>>,
	#[serde(rename = "recordRollbacks")]
	pub record_rollbacks: Option<Vec<FlowRecordRollback>>,
	#[serde(rename = "recordUpdates")]
	pub record_updates: Option<Vec<FlowRecordUpdate>>,
	#[serde(rename = "runInMode")]
	pub run_in_mode: Option<FlowRunInMode>,
	#[serde(rename = "screens")]
	pub screens: Option<Vec<FlowScreen>>,
	#[serde(rename = "sourceTemplate")]
	pub source_template: Option<String>,
	#[serde(rename = "stages")]
	pub stages: Option<Vec<FlowStage>>,
	#[serde(rename = "start")]
	pub start: Option<FlowStart>,
	#[serde(rename = "startElementReference")]
	pub start_element_reference: Option<String>,
	#[serde(rename = "status")]
	pub status: Option<FlowVersionStatus>,
	#[serde(rename = "steps")]
	pub steps: Option<Vec<FlowStep>>,
	#[serde(rename = "subflows")]
	pub subflows: Option<Vec<FlowSubflow>>,
	#[serde(rename = "textTemplates")]
	pub text_templates: Option<Vec<FlowTextTemplate>>,
	#[serde(rename = "timeZoneSidKey")]
	pub time_zone_sid_key: Option<String>,
	#[serde(rename = "transforms")]
	pub transforms: Option<Vec<FlowTransform>>,
	#[serde(rename = "triggerOrder")]
	pub trigger_order: Option<i32>,
	#[serde(rename = "variables")]
	pub variables: Option<Vec<FlowVariable>>,
	#[serde(rename = "waits")]
	pub waits: Option<Vec<FlowWait>>,
}
