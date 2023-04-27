fn main() {
    println!("Hello, world!");
    let a = 20;
    let b = 20;
    let c = addition(a, b);
    println!("sum of {0} and {1} is {2}. ", a, b, c);
    let circle = Circle {
		x : 5.0,
		y: 5.0,
		radius: 10.0
	};
	let area_of_circle = circle.area();
	println!("area of a circle with radius {0} is: {1}", circle.radius, area_of_circle);
}

struct Circle {
	x: f64,
	y: f64,
	radius: f64 //comma is optional
}

impl Circle {
	fn area(&self) -> f64 {
		std::f64::consts::PI * (self.radius * self.radius)
	}
}

fn addition(a: i32, b: i32) -> i32 {
	a + b
}
