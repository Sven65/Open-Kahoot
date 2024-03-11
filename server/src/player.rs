pub fn calculate_points(time_taken: f64, max_time: f64, max_points: f64) -> f64 {
	// Calculate the ratio of time taken to max time allowed
	let time_ratio = if time_taken >= max_time {
		0.0 // If time taken exceeds or equals max time, set ratio to 0.0 (minimum)
	} else {
		1.0 - (time_taken as f64 / max_time as f64)
	};

	// Calculate the points inversely proportional to the time taken
	return time_ratio * max_points + 1.0
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct Player {
	pub id: String,
	pub points: f64,
}

impl Player {
	pub fn add_points(&mut self, points: f64) {
		self.points += points;
	}
}