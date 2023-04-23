use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmbeddedServiceLayoutRule  {
	#[serde(rename = "appointmentStatus")]
	pub appointment_status: String,
}
