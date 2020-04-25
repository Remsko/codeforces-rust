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
 
fn get_solution(input: Vec<u32>) {
    let n = input[0];
    let a = input[1];
    let b = input[2];
    let c = input[3];
    let d = input[4];
    let n_min = a - b;
    let n_max = a + b;
    let p_min = c - d;
    let p_max = c + d;
    if n_min * n <= p_max && n_max * n >= p_min {
        println!("Yes");
    } else {
        println!("No");
    }
}
 
fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = get_input_number::<u32>()?;
    while lines > 0 {
        let input = get_input_array::<u32>()?;
        get_solution(input);
        lines -= 1;
    }
    Ok(())
}