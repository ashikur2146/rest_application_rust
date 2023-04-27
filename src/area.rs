pub fn test_external() -> &'static str {
			"<==== External Mod area.rs Has Been Called. ======>"
}

pub struct Circle {
	pub x: f64,
	pub y: f64,
	pub radius: f64
}

impl Circle {
	pub fn area(&self) -> f64 {
		std::f64::consts::PI * (self.radius * self.radius)
	}
}

pub fn addition(a: i32, b: i32) -> i32 {
	a + b
}