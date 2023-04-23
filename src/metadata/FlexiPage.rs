use crate::metadata::FlexiPageEvent::FlexiPageEvent;
use crate::metadata::FlexiPageRegion::FlexiPageRegion;
use crate::metadata::FlexiPageTemplateInstance::FlexiPageTemplateInstance;
use crate::metadata::FlexiPageType::FlexiPageType;
use crate::metadata::PlatformActionList::PlatformActionList;
use crate::metadata::QuickActionList::QuickActionList;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlexiPage  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "events")]
	pub events: Option<Vec<FlexiPageEvent>>,
	#[serde(rename = "flexiPageRegions")]
	pub flexi_page_regions: Option<Vec<FlexiPageRegion>>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "parentFlexiPage")]
	pub parent_flexi_page: Option<String>,
	#[serde(rename = "platformActionlist")]
	pub platform_actionlist: Option<PlatformActionList>,
	#[serde(rename = "quickActionList")]
	pub quick_action_list: Option<QuickActionList>,
	#[serde(rename = "sobjectType")]
	pub sobject_type: Option<String>,
	#[serde(rename = "template")]
	pub template: FlexiPageTemplateInstance,
	#[serde(rename = "type")]
	pub _type: FlexiPageType,
}
