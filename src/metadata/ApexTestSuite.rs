use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ApexTestSuite  {
	#[serde(rename = "testClassName")]
	pub test_class_name: Option<Vec<String>>,
}
