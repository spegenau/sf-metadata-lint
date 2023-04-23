use crate::metadata::EmbeddedServiceFeature::EmbeddedServiceFeature;
use crate::metadata::EmbeddedServiceLabelKey::EmbeddedServiceLabelKey;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmbeddedServiceCustomLabel  {
	#[serde(rename = "customLabel")]
	pub custom_label: Option<String>,
	#[serde(rename = "feature")]
	pub feature: Option<EmbeddedServiceFeature>,
	#[serde(rename = "labelKey")]
	pub label_key: Option<EmbeddedServiceLabelKey>,
}
