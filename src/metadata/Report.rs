use crate::metadata::CurrencyIsoCode::CurrencyIsoCode;
use crate::metadata::ReportAggregate::ReportAggregate;
use crate::metadata::ReportBlockInfo::ReportBlockInfo;
use crate::metadata::ReportBucketField::ReportBucketField;
use crate::metadata::ReportChart::ReportChart;
use crate::metadata::ReportColorRange::ReportColorRange;
use crate::metadata::ReportColumn::ReportColumn;
use crate::metadata::ReportCrossFilter::ReportCrossFilter;
use crate::metadata::ReportCustomDetailFormula::ReportCustomDetailFormula;
use crate::metadata::ReportDataCategoryFilter::ReportDataCategoryFilter;
use crate::metadata::ReportFilter::ReportFilter;
use crate::metadata::ReportFormat::ReportFormat;
use crate::metadata::ReportFormattingRule::ReportFormattingRule;
use crate::metadata::ReportGrouping::ReportGrouping;
use crate::metadata::ReportHistoricalSelector::ReportHistoricalSelector;
use crate::metadata::ReportParam::ReportParam;
use crate::metadata::ReportTimeFrameFilter::ReportTimeFrameFilter;
use crate::metadata::SortOrder::SortOrder;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Report  {
	#[serde(rename = "aggregates")]
	pub aggregates: Option<Vec<ReportAggregate>>,
	#[serde(rename = "block")]
	pub block: Option<Vec<Box<Report>>>,
	#[serde(rename = "blockInfo")]
	pub block_info: Option<ReportBlockInfo>,
	#[serde(rename = "buckets")]
	pub buckets: Option<Vec<ReportBucketField>>,
	#[serde(rename = "chart")]
	pub chart: Option<ReportChart>,
	#[serde(rename = "colorRanges")]
	pub color_ranges: Option<Vec<ReportColorRange>>,
	#[serde(rename = "columns")]
	pub columns: Option<Vec<ReportColumn>>,
	#[serde(rename = "crossFilters")]
	pub cross_filters: Option<Vec<ReportCrossFilter>>,
	#[serde(rename = "currency")]
	pub currency: Option<CurrencyIsoCode>,
	#[serde(rename = "customDetailFormulas")]
	pub custom_detail_formulas: Option<Vec<ReportCustomDetailFormula>>,
	#[serde(rename = "dataCategoryFilters")]
	pub data_category_filters: Option<Vec<ReportDataCategoryFilter>>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "division")]
	pub division: Option<String>,
	#[serde(rename = "filter")]
	pub filter: Option<ReportFilter>,
	#[serde(rename = "folderName")]
	pub folder_name: Option<String>,
	#[serde(rename = "format")]
	pub format: ReportFormat,
	#[serde(rename = "formattingRules")]
	pub formatting_rules: Option<Vec<ReportFormattingRule>>,
	#[serde(rename = "groupingsAcross")]
	pub groupings_across: Option<Vec<ReportGrouping>>,
	#[serde(rename = "groupingsDown")]
	pub groupings_down: Option<Vec<ReportGrouping>>,
	#[serde(rename = "historicalSelector")]
	pub historical_selector: Option<ReportHistoricalSelector>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "numSubscriptions")]
	pub num_subscriptions: Option<i32>,
	#[serde(rename = "params")]
	pub params: Option<Vec<ReportParam>>,
	#[serde(rename = "reportType")]
	pub report_type: String,
	#[serde(rename = "reportTypeApiName")]
	pub report_type_api_name: Option<String>,
	#[serde(rename = "roleHierarchyFilter")]
	pub role_hierarchy_filter: Option<String>,
	#[serde(rename = "rowLimit")]
	pub row_limit: Option<i32>,
	#[serde(rename = "scope")]
	pub scope: Option<String>,
	#[serde(rename = "showCurrentDate")]
	pub show_current_date: Option<bool>,
	#[serde(rename = "showDetails")]
	pub show_details: Option<bool>,
	#[serde(rename = "showGrandTotal")]
	pub show_grand_total: Option<bool>,
	#[serde(rename = "showSubTotals")]
	pub show_sub_totals: Option<bool>,
	#[serde(rename = "sortColumn")]
	pub sort_column: Option<String>,
	#[serde(rename = "sortOrder")]
	pub sort_order: Option<SortOrder>,
	#[serde(rename = "territoryHierarchyFilter")]
	pub territory_hierarchy_filter: Option<String>,
	#[serde(rename = "timeFrameFilter")]
	pub time_frame_filter: Option<ReportTimeFrameFilter>,
	#[serde(rename = "userFilter")]
	pub user_filter: Option<String>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
