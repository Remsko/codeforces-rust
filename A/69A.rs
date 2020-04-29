use std::error::Error;
use std::io;
use std::str::FromStr;

fn get_line() -> io::Result<String> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

#[allow(dead_code)]
fn get_input_number<T>() -> Result<T, Box<dyn Error + 'static>>
where
	T: FromStr,
	<T as FromStr>::Err: Error + 'static,
{
	Ok(get_line()?.trim().parse::<T>()?)
}

#[allow(dead_code)]
fn get_input_array<T>() -> Result<Vec<T>, Box<dyn Error + 'static>>
where
	T: FromStr,
	<T as FromStr>::Err: Error + 'static,
{
	Ok(get_line()?
		.split_whitespace()
		.map(|s| s.parse::<T>())
		.collect::<Result<Vec<_>, _>>()?)
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut lines = get_input_number::<u8>()?;
	let mut x_sum = 0;
	let mut y_sum = 0;
	let mut z_sum = 0;
	while lines > 0 {
		let xyz = get_input_array::<i16>()?;
		x_sum += xyz[0];
		y_sum += xyz[1];
		z_sum += xyz[2];
		lines -= 1;
	}
	println!(
		"{}",
		if x_sum == 0 && y_sum == 0 && z_sum == 0 {
			"YES"
		} else {
			"NO"
		}
	);
	Ok(())
}
