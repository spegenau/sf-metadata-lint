use crate::metadata::ServiceChannelFieldPriority::ServiceChannelFieldPriority;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ServiceChannel  {
	#[serde(rename = "doesMinimizeWidgetOnAccept")]
	pub does_minimize_widget_on_accept: Option<bool>,
	#[serde(rename = "hasAutoAcceptEnabled")]
	pub has_auto_accept_enabled: Option<bool>,
	#[serde(rename = "interactionComponent")]
	pub interaction_component: Option<String>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "relatedEntityType")]
	pub related_entity_type: String,
	#[serde(rename = "secondaryRoutingPriorityField")]
	pub secondary_routing_priority_field: Option<String>,
	#[serde(rename = "serviceChannelFieldPriorities")]
	pub service_channel_field_priorities: Option<Vec<ServiceChannelFieldPriority>>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
