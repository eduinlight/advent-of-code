use std::io::{self};

fn get_digit_from_line(line: &str, str_digits: &Vec<String>) -> u32 {
  let mut best: usize = usize::max_value();
  let mut digit: u32 = 0;
  for (index, str_digit) in str_digits.iter().enumerate() {
    if let Some(pos) = line.find(str_digit) {
      if best > pos {
        best = pos;
        digit = index as u32 + 1;
      }
    }
  }
  for (index, c) in line.char_indices() {
    if c.is_digit(10) && best > index {
      best = index;
      digit = c.to_digit(10).unwrap();
    }
  }
  digit
}

fn main() -> io::Result<()> {
  let mut line = String::new();
  let mut sum: u32 = 0;
  let str_digits = vec![
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
  ]
  .into_iter()
  .map(|d| d.to_string())
  .collect::<Vec<String>>();
  let str_rev_digits = str_digits
    .iter()
    .map(|d| d.chars().rev().collect::<String>())
    .collect();
  loop {
    let bytes = io::stdin().read_line(&mut line)?;
    if bytes == 0 {
      break;
    }
    let first_digit = get_digit_from_line(line.chars().as_str(), &str_digits);
    let second_digit = get_digit_from_line(
      line.chars().rev().collect::<String>().as_str(),
      &str_rev_digits,
    );
    sum = sum + (first_digit * 10 + second_digit);
    line.clear();
  }
  println!("{}", sum);
  Ok(())
}
