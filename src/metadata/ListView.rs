use crate::metadata::FilterScope::FilterScope;
use crate::metadata::Language::Language;
use crate::metadata::ListViewFilter::ListViewFilter;
use crate::metadata::SharedTo::SharedTo;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ListView  {
	#[serde(rename = "booleanFilter")]
	pub boolean_filter: Option<String>,
	#[serde(rename = "columns")]
	pub columns: Option<Vec<String>>,
	#[serde(rename = "division")]
	pub division: Option<String>,
	#[serde(rename = "filterScope")]
	pub filter_scope: FilterScope,
	#[serde(rename = "filters")]
	pub filters: Option<Vec<ListViewFilter>>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "language")]
	pub language: Option<Language>,
	#[serde(rename = "queue")]
	pub queue: Option<String>,
	#[serde(rename = "sharedTo")]
	pub shared_to: Option<SharedTo>,
}
