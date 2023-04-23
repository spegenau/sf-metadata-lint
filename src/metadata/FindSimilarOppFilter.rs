use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FindSimilarOppFilter  {
	#[serde(rename = "similarOpportunitiesDisplayColumns")]
	pub similar_opportunities_display_columns: Option<Vec<String>>,
	#[serde(rename = "similarOpportunitiesMatchFields")]
	pub similar_opportunities_match_fields: Option<Vec<String>>,
}
