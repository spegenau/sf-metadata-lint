use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CareRequestRecords  {
	#[serde(rename = "careRequestRecord")]
	pub care_request_record: String,
}
