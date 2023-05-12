use crate::metadata::WaveXmdDate::WaveXmdDate;
use crate::metadata::WaveXmdDimension::WaveXmdDimension;
use crate::metadata::WaveXmdMeasure::WaveXmdMeasure;
use crate::metadata::WaveXmdOrganization::WaveXmdOrganization;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WaveXmd  {
	#[serde(rename = "application")]
	pub application: Option<String>,
	#[serde(rename = "dataset")]
	pub dataset: String,
	#[serde(rename = "datasetConnector")]
	pub dataset_connector: Option<String>,
	#[serde(rename = "datasetFullyQualifiedName")]
	pub dataset_fully_qualified_name: Option<String>,
	#[serde(rename = "dates")]
	pub dates: Option<Vec<WaveXmdDate>>,
	#[serde(rename = "dimensions")]
	pub dimensions: Option<Vec<WaveXmdDimension>>,
	#[serde(rename = "measures")]
	pub measures: Option<Vec<WaveXmdMeasure>>,
	#[serde(rename = "organizations")]
	pub organizations: Option<Vec<WaveXmdOrganization>>,
	#[serde(rename = "origin")]
	pub origin: Option<String>,
	#[serde(rename = "type")]
	pub _type: Option<String>,
	#[serde(rename = "waveVisualization")]
	pub wave_visualization: Option<String>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
