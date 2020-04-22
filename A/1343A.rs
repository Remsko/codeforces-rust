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

fn get_solution(n: usize) -> usize {
	for pw in 2..30 {
		let val = (1 << pw) - 1;
		if n % val == 0 {
			return n / val;
		}
	}
	panic!("No answer");
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut lines = get_input_number::<usize>()?;
	while lines > 0 {
		let input = get_input_number::<usize>()?;
		let val = get_solution(input);
		println!("{}", val);
		lines -= 1;
	}
	Ok(())
}
