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

fn is_peak(arr: &Vec<usize>, i: usize) -> bool {
	arr[i - 1] < arr[i] && arr[i] > arr[i + 1]
}

fn get_solution(k: usize, heights: Vec<usize>) -> (usize, usize) {
	let mut peaks = 0;
	let mut range = (0, 0 + k - 1);
	//println!("range: {:?}", range);
	for i in range.0 + 1..range.1 {
		if is_peak(&heights, i) {
			peaks += 1;
		}
	}
	let mut minl = 0;
	let mut maxp = peaks;
	while {
		range.0 += 1;
		range.1 += 1;
		range.1 < heights.len()
	} {
		//println!("range {:?}", range);
		let waspeak = is_peak(&heights, range.0);
		let ispeak = is_peak(&heights, range.1 - 1);
		peaks += ispeak as i32 - waspeak as i32;
		if peaks > maxp {
			maxp = peaks;
			minl = range.0;
		}
	}
	(maxp as usize + 1, minl + 1)
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut lines = get_input_number::<u16>()?;
	while lines > 0 {
		let k = get_input_array::<usize>()?[1];
		let heights = get_input_array::<usize>()?;
		let (t, l) = get_solution(k, heights);
		println!("{} {}", t, l);
		lines -= 1;
	}
	Ok(())
}
