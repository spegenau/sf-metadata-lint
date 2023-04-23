use crate::metadata::QueueRoutingConfigSkill::QueueRoutingConfigSkill;
use crate::metadata::RoutingModel::RoutingModel;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct QueueRoutingConfig  {
	#[serde(rename = "capacityPercentage")]
	pub capacity_percentage: Option<f32>,
	#[serde(rename = "capacityWeight")]
	pub capacity_weight: Option<f32>,
	#[serde(rename = "dropAdditionalSkillsTimeout")]
	pub drop_additional_skills_timeout: Option<i32>,
	#[serde(rename = "isAttributeBased")]
	pub is_attribute_based: Option<bool>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "pushTimeout")]
	pub push_timeout: Option<i32>,
	#[serde(rename = "queueOverflowAssignee")]
	pub queue_overflow_assignee: Option<String>,
	#[serde(rename = "routingModel")]
	pub routing_model: RoutingModel,
	#[serde(rename = "routingPriority")]
	pub routing_priority: i32,
	#[serde(rename = "skills")]
	pub skills: Option<Vec<QueueRoutingConfigSkill>>,
	#[serde(rename = "userOverflowAssignee")]
	pub user_overflow_assignee: Option<String>,
}
