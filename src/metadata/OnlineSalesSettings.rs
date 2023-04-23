use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OnlineSalesSettings  {
	#[serde(rename = "enableSubscriptionAppEnrolled")]
	pub enable_subscription_app_enrolled: Option<bool>,
}
