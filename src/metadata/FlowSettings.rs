use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowSettings  {
	#[serde(rename = "canDebugFlowAsAnotherUser")]
	pub can_debug_flow_as_another_user: Option<bool>,
	#[serde(rename = "doesEnforceApexCpuTimeLimit")]
	pub does_enforce_apex_cpu_time_limit: Option<bool>,
	#[serde(rename = "doesFormulaEnforceDataAccess")]
	pub does_formula_enforce_data_access: Option<bool>,
	#[serde(rename = "doesFormulaGenerateHtmlOutput")]
	pub does_formula_generate_html_output: Option<bool>,
	#[serde(rename = "enableFlowBREncodedFixEnabled")]
	pub enable_flow_br_encoded_fix_enabled: Option<bool>,
	#[serde(rename = "enableFlowCustomPropertyEditor")]
	pub enable_flow_custom_property_editor: Option<bool>,
	#[serde(rename = "enableFlowDeployAsActiveEnabled")]
	pub enable_flow_deploy_as_active_enabled: Option<bool>,
	#[serde(rename = "enableFlowFieldFilterEnabled")]
	pub enable_flow_field_filter_enabled: Option<bool>,
	#[serde(rename = "enableFlowFormulasFixEnabled")]
	pub enable_flow_formulas_fix_enabled: Option<bool>,
	#[serde(rename = "enableFlowInterviewSharingEnabled")]
	pub enable_flow_interview_sharing_enabled: Option<bool>,
	#[serde(rename = "enableFlowNullPreviousValueFix")]
	pub enable_flow_null_previous_value_fix: Option<bool>,
	#[serde(rename = "enableFlowPauseEnabled")]
	pub enable_flow_pause_enabled: Option<bool>,
	#[serde(rename = "enableFlowReactiveScreens")]
	pub enable_flow_reactive_screens: Option<bool>,
	#[serde(rename = "enableFlowUseApexExceptionEmail")]
	pub enable_flow_use_apex_exception_email: Option<bool>,
	#[serde(rename = "enableFlowViaRestUsesUserCtxt")]
	pub enable_flow_via_rest_uses_user_ctxt: Option<bool>,
	#[serde(rename = "enableLightningRuntimeEnabled")]
	pub enable_lightning_runtime_enabled: Option<bool>,
	#[serde(rename = "isAccessToInvokedApexRequired")]
	pub is_access_to_invoked_apex_required: Option<bool>,
	#[serde(rename = "isApexPluginAccessModifierRespected")]
	pub is_apex_plugin_access_modifier_respected: Option<bool>,
	#[serde(rename = "isEnhancedFlowListViewVisible")]
	pub is_enhanced_flow_list_view_visible: Option<bool>,
	#[serde(rename = "isFlowApexContextRetired")]
	pub is_flow_apex_context_retired: Option<bool>,
	#[serde(rename = "isFlowBlockAccessToSessionIDEnabled")]
	pub is_flow_block_access_to_session_id_enabled: Option<bool>,
	#[serde(rename = "isManageFlowRequiredForAutomationCharts")]
	pub is_manage_flow_required_for_automation_charts: Option<bool>,
	#[serde(rename = "isSupportRollbackOnErrorForApexInvocableActionsEnabled")]
	pub is_support_rollback_on_error_for_apex_invocable_actions_enabled: Option<bool>,
	#[serde(rename = "isTimeResumedInSameRunContext")]
	pub is_time_resumed_in_same_run_context: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
