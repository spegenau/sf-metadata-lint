use crate::metadata::RealTimeEvent::RealTimeEvent;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RealTimeEventSettings  {
	#[serde(rename = "realTimeEvents")]
	pub real_time_events: Option<Vec<RealTimeEvent>>,
}
