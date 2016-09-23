use std::io::prelude::*;
use std::io;

mod temperature;

fn main() {
	println!("Fahrenheit to Celsius converter");
	println!("");

	loop {
		print!("Fahrenheit value:");
		io::stdout().flush().ok().expect("Could not flush stdout");

		let mut fahrenheit = String::new();

		io::stdin().read_line(&mut fahrenheit)
		    .expect("Failed to read line.");

		let fahrenheit = fahrenheit.trim();

		if fahrenheit == "" {
			break;
		}

		let fahrenheit: f32 = match fahrenheit.parse::<f32>() {
			Ok(num) => num,
			Err(_)  => {
				println!("You didn't enter a number. Please enter a number.");
				println!("");
				continue;
			}
		};

		println!("F: {0}, C: {1}", fahrenheit, temperature::f_to_c(fahrenheit));
	}
}

