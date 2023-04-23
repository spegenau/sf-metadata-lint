use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReportAggregateReference  {
	#[serde(rename = "aggregate")]
	pub aggregate: String,
}
