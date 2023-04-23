use crate::metadata::CountryIsoCode::CountryIsoCode;
use crate::metadata::OrderLifeCycleType::OrderLifeCycleType;
use crate::metadata::PricingStrategy::PricingStrategy;
use crate::metadata::ProductGrouping::ProductGrouping;
use crate::metadata::TaxLocaleType::TaxLocaleType;
use crate::metadata::WebStoreType::WebStoreType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WebStoreTemplate  {
	#[serde(rename = "autoFacetingEnabled")]
	pub auto_faceting_enabled: Option<bool>,
	#[serde(rename = "cartToOrderAutoCustomFieldMapping")]
	pub cart_to_order_auto_custom_field_mapping: Option<bool>,
	#[serde(rename = "checkoutTimeToLive")]
	pub checkout_time_to_live: Option<i32>,
	#[serde(rename = "checkoutValidAfterDate")]
	pub checkout_valid_after_date: Option<String>,
	#[serde(rename = "commerceEinsteinActivitiesTracked")]
	pub commerce_einstein_activities_tracked: Option<bool>,
	#[serde(rename = "commerceEinsteinDeployed")]
	pub commerce_einstein_deployed: Option<bool>,
	#[serde(rename = "country")]
	pub country: Option<CountryIsoCode>,
	#[serde(rename = "defaultCurrency")]
	pub default_currency: Option<String>,
	#[serde(rename = "defaultLanguage")]
	pub default_language: String,
	#[serde(rename = "defaultTaxLocaleType")]
	pub default_tax_locale_type: TaxLocaleType,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "guestBrowsingEnabled")]
	pub guest_browsing_enabled: Option<bool>,
	#[serde(rename = "guestCartTimeToLive")]
	pub guest_cart_time_to_live: Option<i32>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "maxValuesPerFacet")]
	pub max_values_per_facet: Option<i32>,
	#[serde(rename = "orderActivationStatus")]
	pub order_activation_status: Option<String>,
	#[serde(rename = "orderLifeCycleType")]
	pub order_life_cycle_type: Option<OrderLifeCycleType>,
	#[serde(rename = "paginationSize")]
	pub pagination_size: Option<i32>,
	#[serde(rename = "pricingStrategy")]
	pub pricing_strategy: PricingStrategy,
	#[serde(rename = "productGrouping")]
	pub product_grouping: Option<ProductGrouping>,
	#[serde(rename = "skipAdditionalEntitlementCheckForSearch")]
	pub skip_additional_entitlement_check_for_search: Option<bool>,
	#[serde(rename = "supportedCurrencies")]
	pub supported_currencies: Option<String>,
	#[serde(rename = "supportedLanguages")]
	pub supported_languages: String,
	#[serde(rename = "supportedShipToCountries")]
	pub supported_ship_to_countries: Option<String>,
	#[serde(rename = "type")]
	pub _type: WebStoreType,
}
