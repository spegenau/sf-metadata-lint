use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct IndustriesLoyaltySettings  {
	#[serde(rename = "enableAutomaticVoucherCodeGeneration")]
	pub enable_automatic_voucher_code_generation: Option<bool>,
	#[serde(rename = "enableFixedTypeNQPAggregation")]
	pub enable_fixed_type_nqp_aggregation: Option<bool>,
	#[serde(rename = "enableLoyaltyRedeemedPointsExpirationInfoPref")]
	pub enable_loyalty_redeemed_points_expiration_info_pref: Option<bool>,
	#[serde(rename = "enableLoyaltyRulesVerifyCdpMemberSegment")]
	pub enable_loyalty_rules_verify_cdp_member_segment: Option<bool>,
	#[serde(rename = "enableLoyaltyServiceExcellence")]
	pub enable_loyalty_service_excellence: Option<bool>,
	#[serde(rename = "enableNQPRealTimePointBalance")]
	pub enable_nqp_real_time_point_balance: Option<bool>,
	#[serde(rename = "enableQPRealTimePointBalance")]
	pub enable_qp_real_time_point_balance: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
