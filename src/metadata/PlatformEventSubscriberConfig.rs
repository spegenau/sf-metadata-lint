use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PlatformEventSubscriberConfig  {
	#[serde(rename = "batchSize")]
	pub batch_size: Option<i32>,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "platformEventConsumer")]
	pub platform_event_consumer: String,
	#[serde(rename = "user")]
	pub user: Option<String>,
}
