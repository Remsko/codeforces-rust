use std::error::Error;
use std::io;
use std::str::FromStr;

fn get_lines() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
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


#[inline(always)]
fn manhattan_distance(xa: i8, xb: i8, ya: i8, yb: i8) -> i8 {
    i8::abs(xb - xa) + i8::abs(yb - ya)
}
 
fn main() -> Result<(), Box<dyn Error>> {
    let mut x = 0;
    let mut y = 0;
    for i in 0..5 {
        let rows: Vec<u8> = get_input_array();
        for j in 0..5 {
            if rows[j] == 1 {
                x = j;
                y = i;
            }
        }
    }
    println!("{}", manhattan_distance(x as i8, 2, y as i8, 2));
    Ok(())
}