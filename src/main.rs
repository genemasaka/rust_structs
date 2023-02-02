use std::io;

struct Vehicle {
	make: String,
	model: String,
	year: usize,
	milage: usize,
}

fn main() {
	println!("Please fill in the fields to build a car:\n");
	println!("make:\n");
	let mut make = String::new();
	io::stdin().read_line(&mut make).expect("Failed to read input");
	println!("model:\n");
	let mut model = String::new();
	io::stdin().read_line(&mut model).expect("Failed to read input");
	println!("year:\n");
	let mut year = String::new();
	io::stdin().read_line(&mut year).expect("Failed to read input");
	let parsed_year: usize = year.parse::<usize>().unwrap();
	println!("milage:\n");
	let mut milage = String::new();
	io::stdin().read_line(&mut milage).expect("Failed to read input");
	let parsed_milage: usize = milage.parse::<usize>().unwrap();

	let new_car = build_car(make, model, parsed_year, parsed_milage);

	println!("{}", new_car.milage);
}

fn build_car(make: String, model: String, year: usize, milage: usize) -> Vehicle{
	let car = Vehicle {
		make: make,
		model: model,
		year: year,
		milage: milage,
	};

	car
}