use std::error::Error;
use std::io;
use std::str::FromStr;

fn get_line() -> io::Result<String> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn get_input_number<T>() -> Result<T, Box<dyn Error + 'static>>
where
	T: FromStr,
	<T as FromStr>::Err: Error + 'static,
{
	Ok(get_line()?.trim().parse::<T>()?)
}

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
	let input = get_input_array::<usize>()?;
	println!("{}",);
	Ok(())
}
