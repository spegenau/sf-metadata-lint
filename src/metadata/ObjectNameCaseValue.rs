use crate::metadata::Article::Article;
use crate::metadata::CaseType::CaseType;
use crate::metadata::Possessive::Possessive;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ObjectNameCaseValue  {
	#[serde(rename = "article")]
	pub article: Option<Article>,
	#[serde(rename = "caseType")]
	pub case_type: Option<CaseType>,
	#[serde(rename = "plural")]
	pub plural: Option<bool>,
	#[serde(rename = "possessive")]
	pub possessive: Option<Possessive>,
	#[serde(rename = "value")]
	pub value: String,
}
