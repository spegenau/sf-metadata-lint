use crate::metadata::ExtendedErrorCode::ExtendedErrorCode;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ExtendedErrorDetails  {
	#[serde(rename = "extendedErrorCode")]
	pub extended_error_code: ExtendedErrorCode,
}
