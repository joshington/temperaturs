//===temperature has to be mutable
//

use std::io;

fn main() {
	loop {
		println!("Enter temperature in Celsius");
		let mut  temperature = String::new();
		io::stdin().read_line(&mut temperature)
			.expect("Failed to read line");
		//after reading from the line
		let farenheit: i32 = match temperature.trim().parse() {
			Ok(farenheit) => farenheit,
			Err(_) => {
				println!("Invalid, input must be a number not atsring");
				return;
			}
		};
		let farenheit = farenheit + 32;
		println!("Temp in Farenheit, {}",farenheit); 
	}
	 
}
