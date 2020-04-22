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

fn get_solution(mut n: u64) {
	if !(n % 4 == 0) {
		println!("NO");
		return;
	}
	println!("YES");
	n /= 2;
	for i in 1..=n {
		print!("{} ", i * 2);
	}
	for i in 1..n {
		print!("{} ", i * 2 - 1);
	}
	println!("{}", 3 * n - 1);
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut lines = get_input_number::<u64>()?;
	while lines > 0 {
		let input = get_input_number::<u64>()?;
		get_solution(input);
		lines -= 1;
	}
	Ok(())
}
