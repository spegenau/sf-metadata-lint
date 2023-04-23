use crate::metadata::FindSimilarOppFilter::FindSimilarOppFilter;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OpportunitySettings  {
	#[serde(rename = "autoActivateNewReminders")]
	pub auto_activate_new_reminders: Option<bool>,
	#[serde(rename = "customizableProductSchedulesEnabled")]
	pub customizable_product_schedules_enabled: Option<bool>,
	#[serde(rename = "doesEnforceStandardOpportunitySaveLogic")]
	pub does_enforce_standard_opportunity_save_logic: Option<bool>,
	#[serde(rename = "enableExpandedPipelineInspectionSetup")]
	pub enable_expanded_pipeline_inspection_setup: Option<bool>,
	#[serde(rename = "enableFindSimilarOpportunities")]
	pub enable_find_similar_opportunities: Option<bool>,
	#[serde(rename = "enableForecastCategoryMetrics")]
	pub enable_forecast_category_metrics: Option<bool>,
	#[serde(rename = "enableOpportunityFieldHistoryTracking")]
	pub enable_opportunity_field_history_tracking: Option<bool>,
	#[serde(rename = "enableOpportunityInsightsInMobile")]
	pub enable_opportunity_insights_in_mobile: Option<bool>,
	#[serde(rename = "enableOpportunityTeam")]
	pub enable_opportunity_team: Option<bool>,
	#[serde(rename = "enablePipelineChangesMetrics")]
	pub enable_pipeline_changes_metrics: Option<bool>,
	#[serde(rename = "enablePipelineInspection")]
	pub enable_pipeline_inspection: Option<bool>,
	#[serde(rename = "enablePipelineInspectionFlow")]
	pub enable_pipeline_inspection_flow: Option<bool>,
	#[serde(rename = "enablePipelineInspectionSingleCategoryRollup")]
	pub enable_pipeline_inspection_single_category_rollup: Option<bool>,
	#[serde(rename = "enableRevenueInsights")]
	pub enable_revenue_insights: Option<bool>,
	#[serde(rename = "enableServiceCaseInsights")]
	pub enable_service_case_insights: Option<bool>,
	#[serde(rename = "enableUpdateReminders")]
	pub enable_update_reminders: Option<bool>,
	#[serde(rename = "findSimilarOppFilter")]
	pub find_similar_opp_filter: Option<FindSimilarOppFilter>,
	#[serde(rename = "oppAmountDealMotionEnabled")]
	pub opp_amount_deal_motion_enabled: Option<bool>,
	#[serde(rename = "oppCloseDateDealMotionEnabled")]
	pub opp_close_date_deal_motion_enabled: Option<bool>,
	#[serde(rename = "promptToAddProducts")]
	pub prompt_to_add_products: Option<bool>,
	#[serde(rename = "pushCountEnabled")]
	pub push_count_enabled: Option<bool>,
	#[serde(rename = "simpleOppCreateFromContact")]
	pub simple_opp_create_from_contact: Option<bool>,
	#[serde(rename = "simpleOppCreateFromEvent")]
	pub simple_opp_create_from_event: Option<bool>,
}
