use crate::metadata::ActionOverride::ActionOverride;
use crate::metadata::ArticleTypeChannelDisplay::ArticleTypeChannelDisplay;
use crate::metadata::BusinessProcess::BusinessProcess;
use crate::metadata::CompactLayout::CompactLayout;
use crate::metadata::CustomField::CustomField;
use crate::metadata::CustomSettingsType::CustomSettingsType;
use crate::metadata::DeploymentStatus::DeploymentStatus;
use crate::metadata::FieldSet::FieldSet;
use crate::metadata::Gender::Gender;
use crate::metadata::HistoryRetentionPolicy::HistoryRetentionPolicy;
use crate::metadata::Index::Index;
use crate::metadata::ListView::ListView;
use crate::metadata::MktDataLakeAttributes::MktDataLakeAttributes;
use crate::metadata::MktDataModelAttributes::MktDataModelAttributes;
use crate::metadata::PlatformEventPublishBehavior::PlatformEventPublishBehavior;
use crate::metadata::PlatformEventType::PlatformEventType;
use crate::metadata::ProfileSearchLayouts::ProfileSearchLayouts;
use crate::metadata::RecordType::RecordType;
use crate::metadata::SearchLayouts::SearchLayouts;
use crate::metadata::SetupObjectVisibility::SetupObjectVisibility;
use crate::metadata::SharingModel::SharingModel;
use crate::metadata::SharingReason::SharingReason;
use crate::metadata::SharingRecalculation::SharingRecalculation;
use crate::metadata::StartsWith::StartsWith;
use crate::metadata::ValidationRule::ValidationRule;
use crate::metadata::WebLink::WebLink;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomObject  {
	#[serde(rename = "actionOverrides")]
	pub action_overrides: Option<Vec<ActionOverride>>,
	#[serde(rename = "allowInChatterGroups")]
	pub allow_in_chatter_groups: Option<bool>,
	#[serde(rename = "articleTypeChannelDisplay")]
	pub article_type_channel_display: Option<ArticleTypeChannelDisplay>,
	#[serde(rename = "businessProcesses")]
	pub business_processes: Option<Vec<BusinessProcess>>,
	#[serde(rename = "compactLayoutAssignment")]
	pub compact_layout_assignment: Option<String>,
	#[serde(rename = "compactLayouts")]
	pub compact_layouts: Option<Vec<CompactLayout>>,
	#[serde(rename = "customHelp")]
	pub custom_help: Option<String>,
	#[serde(rename = "customHelpPage")]
	pub custom_help_page: Option<String>,
	#[serde(rename = "customSettingsType")]
	pub custom_settings_type: Option<CustomSettingsType>,
	#[serde(rename = "deploymentStatus")]
	pub deployment_status: Option<DeploymentStatus>,
	#[serde(rename = "deprecated")]
	pub deprecated: Option<bool>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "enableActivities")]
	pub enable_activities: Option<bool>,
	#[serde(rename = "enableBulkApi")]
	pub enable_bulk_api: Option<bool>,
	#[serde(rename = "enableDataTranslation")]
	pub enable_data_translation: Option<bool>,
	#[serde(rename = "enableDivisions")]
	pub enable_divisions: Option<bool>,
	#[serde(rename = "enableEnhancedLookup")]
	pub enable_enhanced_lookup: Option<bool>,
	#[serde(rename = "enableFeeds")]
	pub enable_feeds: Option<bool>,
	#[serde(rename = "enableHistory")]
	pub enable_history: Option<bool>,
	#[serde(rename = "enableLicensing")]
	pub enable_licensing: Option<bool>,
	#[serde(rename = "enablePublishStatusTracking")]
	pub enable_publish_status_tracking: Option<bool>,
	#[serde(rename = "enableReports")]
	pub enable_reports: Option<bool>,
	#[serde(rename = "enableSearch")]
	pub enable_search: Option<bool>,
	#[serde(rename = "enableSharing")]
	pub enable_sharing: Option<bool>,
	#[serde(rename = "enableStreamingApi")]
	pub enable_streaming_api: Option<bool>,
	#[serde(rename = "eventType")]
	pub event_type: Option<PlatformEventType>,
	#[serde(rename = "externalDataSource")]
	pub external_data_source: Option<String>,
	#[serde(rename = "externalName")]
	pub external_name: Option<String>,
	#[serde(rename = "externalRepository")]
	pub external_repository: Option<String>,
	#[serde(rename = "externalSharingModel")]
	pub external_sharing_model: Option<SharingModel>,
	#[serde(rename = "fieldSets")]
	pub field_sets: Option<Vec<FieldSet>>,
	#[serde(rename = "fields")]
	pub fields: Option<Vec<CustomField>>,
	#[serde(rename = "gender")]
	pub gender: Option<Gender>,
	#[serde(rename = "historyRetentionPolicy")]
	pub history_retention_policy: Option<HistoryRetentionPolicy>,
	#[serde(rename = "household")]
	pub household: Option<bool>,
	#[serde(rename = "indexes")]
	pub indexes: Option<Vec<Index>>,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "listViews")]
	pub list_views: Option<Vec<ListView>>,
	#[serde(rename = "mktDataLakeAttributes")]
	pub mkt_data_lake_attributes: Option<MktDataLakeAttributes>,
	#[serde(rename = "mktDataModelAttributes")]
	pub mkt_data_model_attributes: Option<MktDataModelAttributes>,
	#[serde(rename = "nameField")]
	pub name_field: Option<CustomField>,
	#[serde(rename = "pluralLabel")]
	pub plural_label: Option<String>,
	#[serde(rename = "profileSearchLayouts")]
	pub profile_search_layouts: Option<Vec<ProfileSearchLayouts>>,
	#[serde(rename = "publishBehavior")]
	pub publish_behavior: Option<PlatformEventPublishBehavior>,
	#[serde(rename = "recordTypeTrackFeedHistory")]
	pub record_type_track_feed_history: Option<bool>,
	#[serde(rename = "recordTypeTrackHistory")]
	pub record_type_track_history: Option<bool>,
	#[serde(rename = "recordTypes")]
	pub record_types: Option<Vec<RecordType>>,
	#[serde(rename = "searchLayouts")]
	pub search_layouts: Option<SearchLayouts>,
	#[serde(rename = "sharingModel")]
	pub sharing_model: Option<SharingModel>,
	#[serde(rename = "sharingReasons")]
	pub sharing_reasons: Option<Vec<SharingReason>>,
	#[serde(rename = "sharingRecalculations")]
	pub sharing_recalculations: Option<Vec<SharingRecalculation>>,
	#[serde(rename = "startsWith")]
	pub starts_with: Option<StartsWith>,
	#[serde(rename = "validationRules")]
	pub validation_rules: Option<Vec<ValidationRule>>,
	#[serde(rename = "visibility")]
	pub visibility: Option<SetupObjectVisibility>,
	#[serde(rename = "webLinks")]
	pub web_links: Option<Vec<WebLink>>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
