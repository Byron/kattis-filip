use std::{io, io::Read, str::FromStr};

fn main() -> Result<(), Box<std::error::Error + 'static>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input: String = input.trim().chars().rev().collect();
    let mut nums = input.split(' ').map(usize::from_str).filter_map(Result::ok);
    match (nums.next(), nums.next()) {
        (Some(first), Some(second)) => println!("{}", if first > second { first } else { second }),
        _ => panic!("exhausted"),
    };
    Ok(())
}
