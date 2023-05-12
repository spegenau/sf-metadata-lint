use crate::metadata::ApptAssistantRadiusUnit::ApptAssistantRadiusUnit;
use crate::metadata::ObjectMappingItem::ObjectMappingItem;
use crate::metadata::WorkOrderDurationSource::WorkOrderDurationSource;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FieldServiceSettings  {
	#[serde(rename = "apptAssistantExpiration")]
	pub appt_assistant_expiration: Option<i32>,
	#[serde(rename = "apptAssistantInfoUrl")]
	pub appt_assistant_info_url: Option<String>,
	#[serde(rename = "apptAssistantRadiusUnitValue")]
	pub appt_assistant_radius_unit_value: Option<ApptAssistantRadiusUnit>,
	#[serde(rename = "apptAssistantRadiusValue")]
	pub appt_assistant_radius_value: Option<i32>,
	#[serde(rename = "apptAssistantStatus")]
	pub appt_assistant_status: Option<String>,
	#[serde(rename = "deepLinkPublicSecurityKey")]
	pub deep_link_public_security_key: Option<String>,
	#[serde(rename = "doesAllowEditSaForCrew")]
	pub does_allow_edit_sa_for_crew: Option<bool>,
	#[serde(rename = "doesShareSaParentWoWithAr")]
	pub does_share_sa_parent_wo_with_ar: Option<bool>,
	#[serde(rename = "doesShareSaWithAr")]
	pub does_share_sa_with_ar: Option<bool>,
	#[serde(rename = "enableBatchWindow")]
	pub enable_batch_window: Option<bool>,
	#[serde(rename = "enableFloatingWorkOrder")]
	pub enable_floating_work_order: Option<bool>,
	#[serde(rename = "enablePopulateWorkOrderAddress")]
	pub enable_populate_work_order_address: Option<bool>,
	#[serde(rename = "enableWorkOrders")]
	pub enable_work_orders: Option<bool>,
	#[serde(rename = "enableWorkPlansAutoGeneration")]
	pub enable_work_plans_auto_generation: Option<bool>,
	#[serde(rename = "enableWorkStepManualStatusUpdate")]
	pub enable_work_step_manual_status_update: Option<bool>,
	#[serde(rename = "fieldServiceNotificationsOrgPref")]
	pub field_service_notifications_org_pref: Option<bool>,
	#[serde(rename = "fieldServiceOrgPref")]
	pub field_service_org_pref: Option<bool>,
	#[serde(rename = "isGeoCodeSyncEnabled")]
	pub is_geo_code_sync_enabled: Option<bool>,
	#[serde(rename = "isLocationHistoryEnabled")]
	pub is_location_history_enabled: Option<bool>,
	#[serde(rename = "mobileFeedbackEmails")]
	pub mobile_feedback_emails: Option<String>,
	#[serde(rename = "o2EngineEnabled")]
	pub o_2_engine_enabled: Option<bool>,
	#[serde(rename = "objectMappingItem")]
	pub object_mapping_item: Option<Vec<ObjectMappingItem>>,
	#[serde(rename = "optimizationServiceAccess")]
	pub optimization_service_access: Option<bool>,
	#[serde(rename = "serviceAppointmentsDueDateOffsetOrgValue")]
	pub service_appointments_due_date_offset_org_value: Option<i32>,
	#[serde(rename = "workOrderDurationSource")]
	pub work_order_duration_source: Option<WorkOrderDurationSource>,
	#[serde(rename = "workOrderLineItemSearchFields")]
	pub work_order_line_item_search_fields: Option<Vec<String>>,
	#[serde(rename = "workOrderSearchFields")]
	pub work_order_search_fields: Option<Vec<String>>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
