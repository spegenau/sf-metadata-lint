use crate::metadata::LightningBoltCategory::LightningBoltCategory;
use crate::metadata::LightningBoltFeatures::LightningBoltFeatures;
use crate::metadata::LightningBoltImages::LightningBoltImages;
use crate::metadata::LightningBoltItems::LightningBoltItems;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LightningBolt  {
	#[serde(rename = "category")]
	pub category: LightningBoltCategory,
	#[serde(rename = "lightningBoltFeatures")]
	pub lightning_bolt_features: Option<Vec<LightningBoltFeatures>>,
	#[serde(rename = "lightningBoltImages")]
	pub lightning_bolt_images: Option<Vec<LightningBoltImages>>,
	#[serde(rename = "lightningBoltItems")]
	pub lightning_bolt_items: Option<Vec<LightningBoltItems>>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "publisher")]
	pub publisher: String,
	#[serde(rename = "summary")]
	pub summary: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
