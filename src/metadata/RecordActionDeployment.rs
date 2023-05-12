use crate::metadata::ComponentName::ComponentName;
use crate::metadata::RecordActionDeploymentChannel::RecordActionDeploymentChannel;
use crate::metadata::RecordActionDeploymentContext::RecordActionDeploymentContext;
use crate::metadata::RecordActionRecommendation::RecordActionRecommendation;
use crate::metadata::RecordActionSelectableItem::RecordActionSelectableItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RecordActionDeployment  {
	#[serde(rename = "channelConfigurations")]
	pub channel_configurations: Option<Vec<RecordActionDeploymentChannel>>,
	#[serde(rename = "componentName")]
	pub component_name: Option<ComponentName>,
	#[serde(rename = "deploymentContexts")]
	pub deployment_contexts: Option<Vec<RecordActionDeploymentContext>>,
	#[serde(rename = "hasGuidedActions")]
	pub has_guided_actions: Option<bool>,
	#[serde(rename = "hasOmniscripts")]
	pub has_omniscripts: Option<bool>,
	#[serde(rename = "hasRecommendations")]
	pub has_recommendations: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "recommendation")]
	pub recommendation: Option<RecordActionRecommendation>,
	#[serde(rename = "selectableItems")]
	pub selectable_items: Option<Vec<RecordActionSelectableItem>>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
