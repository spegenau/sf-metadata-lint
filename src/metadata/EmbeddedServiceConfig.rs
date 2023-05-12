use crate::metadata::EmbeddedServiceAppointmentSettings::EmbeddedServiceAppointmentSettings;
use crate::metadata::EmbeddedServiceAuthMethod::EmbeddedServiceAuthMethod;
use crate::metadata::EmbeddedServiceCustomComponent::EmbeddedServiceCustomComponent;
use crate::metadata::EmbeddedServiceCustomLabel::EmbeddedServiceCustomLabel;
use crate::metadata::EmbeddedServiceCustomization::EmbeddedServiceCustomization;
use crate::metadata::EmbeddedServiceDeploymentFeature::EmbeddedServiceDeploymentFeature;
use crate::metadata::EmbeddedServiceDeploymentType::EmbeddedServiceDeploymentType;
use crate::metadata::EmbeddedServiceFlow::EmbeddedServiceFlow;
use crate::metadata::EmbeddedServiceFlowConfig::EmbeddedServiceFlowConfig;
use crate::metadata::EmbeddedServiceLayout::EmbeddedServiceLayout;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmbeddedServiceConfig  {
	#[serde(rename = "areGuestUsersAllowed")]
	pub are_guest_users_allowed: Option<bool>,
	#[serde(rename = "authMethod")]
	pub auth_method: Option<EmbeddedServiceAuthMethod>,
	#[serde(rename = "branding")]
	pub branding: Option<String>,
	#[serde(rename = "deploymentFeature")]
	pub deployment_feature: EmbeddedServiceDeploymentFeature,
	#[serde(rename = "deploymentType")]
	pub deployment_type: EmbeddedServiceDeploymentType,
	#[serde(rename = "embeddedServiceAppointmentSettings")]
	pub embedded_service_appointment_settings: Option<EmbeddedServiceAppointmentSettings>,
	#[serde(rename = "embeddedServiceCustomComponents")]
	pub embedded_service_custom_components: Option<Vec<EmbeddedServiceCustomComponent>>,
	#[serde(rename = "embeddedServiceCustomLabels")]
	pub embedded_service_custom_labels: Option<Vec<EmbeddedServiceCustomLabel>>,
	#[serde(rename = "embeddedServiceCustomizations")]
	pub embedded_service_customizations: Option<Vec<EmbeddedServiceCustomization>>,
	#[serde(rename = "embeddedServiceFlowConfig")]
	pub embedded_service_flow_config: Option<EmbeddedServiceFlowConfig>,
	#[serde(rename = "embeddedServiceFlows")]
	pub embedded_service_flows: Option<Vec<EmbeddedServiceFlow>>,
	#[serde(rename = "embeddedServiceLayouts")]
	pub embedded_service_layouts: Option<Vec<EmbeddedServiceLayout>>,
	#[serde(rename = "isEnabled")]
	pub is_enabled: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "shouldHideAuthDialog")]
	pub should_hide_auth_dialog: Option<bool>,
	#[serde(rename = "site")]
	pub site: Option<String>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
