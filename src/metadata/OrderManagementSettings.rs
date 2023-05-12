use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OrderManagementSettings  {
	#[serde(rename = "enableB2CIntegration")]
	pub enable_b_2_c_integration: Option<bool>,
	#[serde(rename = "enableB2CSelfService")]
	pub enable_b_2_c_self_service: Option<bool>,
	#[serde(rename = "enableDuplicateManagement")]
	pub enable_duplicate_management: Option<bool>,
	#[serde(rename = "enableHighScaleOrders")]
	pub enable_high_scale_orders: Option<bool>,
	#[serde(rename = "enableIndividualOrderItemTaxAdjustments")]
	pub enable_individual_order_item_tax_adjustments: Option<bool>,
	#[serde(rename = "enableOMAutomation")]
	pub enable_om_automation: Option<bool>,
	#[serde(rename = "enableOrderManagement")]
	pub enable_order_management: Option<bool>,
	#[serde(rename = "enablePersonAccountsForShoppers")]
	pub enable_person_accounts_for_shoppers: Option<bool>,
	#[serde(rename = "initOMAutomation")]
	pub init_om_automation: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
