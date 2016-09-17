mod temperature;

fn main() {
    println!("Hello, world!");

	let fahrenheit = 32f32;

	println!("F: {0}, C: {1}", fahrenheit, temperature::f_to_c(fahrenheit));

}

