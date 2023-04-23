use crate::metadata::SharedTo::SharedTo;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SharingOwnerRule  {
	#[serde(rename = "sharedFrom")]
	pub shared_from: SharedTo,
}
