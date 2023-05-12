use crate::metadata::QueueMembers::QueueMembers;
use crate::metadata::QueueSobject::QueueSobject;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Queue  {
	#[serde(rename = "doesSendEmailToMembers")]
	pub does_send_email_to_members: Option<bool>,
	#[serde(rename = "email")]
	pub email: Option<String>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "queueMembers")]
	pub queue_members: Option<QueueMembers>,
	#[serde(rename = "queueRoutingConfig")]
	pub queue_routing_config: Option<String>,
	#[serde(rename = "queueSobject")]
	pub queue_sobject: Option<Vec<QueueSobject>>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
