use crate::metadata::AdjustmentsSettings::AdjustmentsSettings;
use crate::metadata::ForecastRangeSettings::ForecastRangeSettings;
use crate::metadata::ForecastingCategoryMapping::ForecastingCategoryMapping;
use crate::metadata::ForecastingDisplayedFamilySettings::ForecastingDisplayedFamilySettings;
use crate::metadata::ForecastingTypeSettings::ForecastingTypeSettings;
use crate::metadata::QuotasSettings::QuotasSettings;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ForecastingSettings  {
	#[serde(rename = "defaultToPersonalCurrency")]
	pub default_to_personal_currency: Option<bool>,
	#[serde(rename = "enableForecasts")]
	pub enable_forecasts: Option<bool>,
	#[serde(rename = "forecastingCategoryMappings")]
	pub forecasting_category_mappings: Option<Vec<ForecastingCategoryMapping>>,
	#[serde(rename = "forecastingDisplayedFamilySettings")]
	pub forecasting_displayed_family_settings: Option<Vec<ForecastingDisplayedFamilySettings>>,
	#[serde(rename = "forecastingTypeSettings")]
	pub forecasting_type_settings: Option<Vec<ForecastingTypeSettings>>,
	#[serde(rename = "globalAdjustmentsSettings")]
	pub global_adjustments_settings: AdjustmentsSettings,
	#[serde(rename = "globalForecastRangeSettings")]
	pub global_forecast_range_settings: ForecastRangeSettings,
	#[serde(rename = "globalQuotasSettings")]
	pub global_quotas_settings: QuotasSettings,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
