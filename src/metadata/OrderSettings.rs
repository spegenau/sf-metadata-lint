use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OrderSettings  {
	#[serde(rename = "enableEnhancedCommerceOrders")]
	pub enable_enhanced_commerce_orders: Option<bool>,
	#[serde(rename = "enableNegativeQuantity")]
	pub enable_negative_quantity: Option<bool>,
	#[serde(rename = "enableOptionalPricebook")]
	pub enable_optional_pricebook: Option<bool>,
	#[serde(rename = "enableOrderEvents")]
	pub enable_order_events: Option<bool>,
	#[serde(rename = "enableOrders")]
	pub enable_orders: bool,
	#[serde(rename = "enableReductionOrders")]
	pub enable_reduction_orders: Option<bool>,
	#[serde(rename = "enableZeroQuantity")]
	pub enable_zero_quantity: Option<bool>,
}
