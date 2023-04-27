fn main() {
    println!("Hello, world!");
    let a = 20;
    let b = 20;
    let c = addition(a, b);
    println!("sum of {0} and {1} is {2}. ", a, b, c);
}

fn addition(a: i32, b: i32) -> i32 {
	a + b
}
