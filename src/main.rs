mod area;

fn main() {
	println!("{}", area::test_external());
    println!("Hello, world!");
    let a = 20;
    let b = 20;
    let c = area::addition(a, b);
    println!("sum of {0} and {1} is {2}. ", a, b, c);
    let circle = area::Circle {
		x : 5.0,
		y: 5.0,
		radius: 10.0
	};
	let area_of_circle = circle.area();
	println!("area of a circle with radius {0} is: {1}", circle.radius, area_of_circle);
}
