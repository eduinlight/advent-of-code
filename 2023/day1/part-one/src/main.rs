use std::io::{self};

fn get_digit_from_line(line: &str) -> Option<u32> {
  for c in line.chars() {
    if c.is_digit(10) {
      return c.to_digit(10);
    }
  }
  None
}

fn main() -> io::Result<()> {
  let mut line = String::new();
  let mut sum: u32 = 0;
  loop {
    let bytes = io::stdin().read_line(&mut line)?;
    if bytes == 0 {
      break;
    }
    let first_digit = get_digit_from_line(line.chars().as_str()).unwrap();
    let second_digit =
      get_digit_from_line(line.chars().rev().collect::<String>().as_str()).unwrap();
    sum = sum + (first_digit * 10 + second_digit);
    line.clear();
  }
  println!("{}", sum);
  Ok(())
}
