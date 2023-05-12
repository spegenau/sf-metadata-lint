use crate::metadata::BotTranslation::BotTranslation;
use crate::metadata::CustomApplicationTranslation::CustomApplicationTranslation;
use crate::metadata::CustomLabelTranslation::CustomLabelTranslation;
use crate::metadata::CustomPageWebLinkTranslation::CustomPageWebLinkTranslation;
use crate::metadata::CustomTabTranslation::CustomTabTranslation;
use crate::metadata::ExplainabilityMsgTemplateFieldTranslation::ExplainabilityMsgTemplateFieldTranslation;
use crate::metadata::FlowDefinitionTranslation::FlowDefinitionTranslation;
use crate::metadata::GlobalQuickActionTranslation::GlobalQuickActionTranslation;
use crate::metadata::IdentityVerificationFieldTranslation::IdentityVerificationFieldTranslation;
use crate::metadata::PipelineInspMetricConfigTranslation::PipelineInspMetricConfigTranslation;
use crate::metadata::PromptTranslation::PromptTranslation;
use crate::metadata::ReportTypeTranslation::ReportTypeTranslation;
use crate::metadata::ScontrolTranslation::ScontrolTranslation;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Translations  {
	#[serde(rename = "bots")]
	pub bots: Option<Vec<BotTranslation>>,
	#[serde(rename = "customApplications")]
	pub custom_applications: Option<Vec<CustomApplicationTranslation>>,
	#[serde(rename = "customLabels")]
	pub custom_labels: Option<Vec<CustomLabelTranslation>>,
	#[serde(rename = "customPageWebLinks")]
	pub custom_page_web_links: Option<Vec<CustomPageWebLinkTranslation>>,
	#[serde(rename = "customTabs")]
	pub custom_tabs: Option<Vec<CustomTabTranslation>>,
	#[serde(rename = "desFieldTemplateMessages")]
	pub des_field_template_messages: Option<Vec<ExplainabilityMsgTemplateFieldTranslation>>,
	#[serde(rename = "flowDefinitions")]
	pub flow_definitions: Option<Vec<FlowDefinitionTranslation>>,
	#[serde(rename = "identityVerificationCustomFieldLabels")]
	pub identity_verification_custom_field_labels: Option<Vec<IdentityVerificationFieldTranslation>>,
	#[serde(rename = "pipelineInspMetricConfigs")]
	pub pipeline_insp_metric_configs: Option<Vec<PipelineInspMetricConfigTranslation>>,
	#[serde(rename = "prompts")]
	pub prompts: Option<Vec<PromptTranslation>>,
	#[serde(rename = "quickActions")]
	pub quick_actions: Option<Vec<GlobalQuickActionTranslation>>,
	#[serde(rename = "reportTypes")]
	pub report_types: Option<Vec<ReportTypeTranslation>>,
	#[serde(rename = "scontrols")]
	pub scontrols: Option<Vec<ScontrolTranslation>>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
