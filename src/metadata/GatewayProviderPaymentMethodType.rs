use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct GatewayProviderPaymentMethodType  {
	#[serde(rename = "comments")]
	pub comments: Option<String>,
	#[serde(rename = "gtwyProviderPaymentMethodType")]
	pub gtwy_provider_payment_method_type: Option<String>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "paymentGatewayProvider")]
	pub payment_gateway_provider: Option<String>,
	#[serde(rename = "paymentMethodType")]
	pub payment_method_type: Option<String>,
	#[serde(rename = "recordType")]
	pub record_type: Option<String>,
}
