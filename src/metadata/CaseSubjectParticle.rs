use crate::metadata::CaseSubjectParticleType::CaseSubjectParticleType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CaseSubjectParticle  {
	#[serde(rename = "index")]
	pub index: i32,
	#[serde(rename = "textField")]
	pub text_field: Option<String>,
	#[serde(rename = "type")]
	pub _type: CaseSubjectParticleType,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
