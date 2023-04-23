use crate::metadata::VirtualVisitComprehendServiceType::VirtualVisitComprehendServiceType;
use crate::metadata::VirtualVisitUsageType::VirtualVisitUsageType;
use crate::metadata::VirtualVisitVisitRegion::VirtualVisitVisitRegion;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct VirtualVisitConfig  {
	#[serde(rename = "comprehendServiceType")]
	pub comprehend_service_type: Option<VirtualVisitComprehendServiceType>,
	#[serde(rename = "experienceCloudSiteUrl")]
	pub experience_cloud_site_url: Option<String>,
	#[serde(rename = "externalMsgServiceIdentifier")]
	pub external_msg_service_identifier: Option<String>,
	#[serde(rename = "externalRoleIdentifier")]
	pub external_role_identifier: Option<String>,
	#[serde(rename = "externalUserIdentifier")]
	pub external_user_identifier: Option<String>,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "messagingRegion")]
	pub messaging_region: Option<String>,
	#[serde(rename = "namedCredential")]
	pub named_credential: Option<String>,
	#[serde(rename = "storageBucketName")]
	pub storage_bucket_name: Option<String>,
	#[serde(rename = "usageType")]
	pub usage_type: Option<VirtualVisitUsageType>,
	#[serde(rename = "videoCallApptTypeValue")]
	pub video_call_appt_type_value: Option<String>,
	#[serde(rename = "videoControlRegion")]
	pub video_control_region: Option<String>,
	#[serde(rename = "visitRegion")]
	pub visit_region: Option<VirtualVisitVisitRegion>,
}
