pub fn test_external() -> &'static str {
			"<==== External Mod area.rs Has Been Called. ======>"
}

pub trait Area {
	fn area(&self) -> f64;
}

pub struct Circle {
	pub x: f64,
	pub y: f64,
	pub radius: f64
}

pub struct Square {
	pub x: f64,
	pub y: f64
}

impl Area for Circle {
	fn area(&self) -> f64 {
		std::f64::consts::PI * (self.radius * self.radius)
	}
}

impl Area for Square {
	fn area(&self) -> f64 {
		self.x * self.y
	}
}

pub fn addition(a: i32, b: i32) -> i32 {
	a + b
}