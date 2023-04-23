use crate::metadata::IdentityVerificationProcDtl::IdentityVerificationProcDtl;
use crate::metadata::IdentityVerificationSearchLayoutType::IdentityVerificationSearchLayoutType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct IdentityVerificationProcDef  {
	#[serde(rename = "identityVerificationProcDtls")]
	pub identity_verification_proc_dtls: Option<Vec<IdentityVerificationProcDtl>>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "searchLayoutType")]
	pub search_layout_type: IdentityVerificationSearchLayoutType,
}
