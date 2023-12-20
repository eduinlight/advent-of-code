use std::io::{self, Result};

fn main() -> Result<()> {
  let mut engine = [[0u8; 1000]; 1000];
  let mut cols: i32 = 0;
  let mut rows: i32 = 0;
  for (i, line) in io::stdin().lines().enumerate() {
    if let Ok(line) = line {
      for (j, c) in line.as_bytes().into_iter().enumerate() {
        engine[i][j] = *c;
      }
      cols = line.len() as i32;
      rows += 1;
    }
  }

  let mut sum: u64 = 0;
  for i in 0..rows {
    for j in 0..cols {
      let c = engine[i as usize][j as usize] as char;
      if c == '*' {
        let mut gear_nums: Vec<u64> = vec![];
        for y in -1..2 {
          for x in -1..2 {
            if y == 0 && x == 0 {
              continue;
            }
            let new_y = i + y;
            let mut new_x = j + x;
            if new_y >= 0
              && new_y < rows
              && new_x >= 0
              && new_x < cols
              && (engine[new_y as usize][new_x as usize] as char).is_digit(10)
            {
              while new_x - 1 >= 0
                && (engine[new_y as usize][(new_x - 1) as usize] as char).is_digit(10)
              {
                new_x -= 1;
              }
              let mut num: u64 = 0;
              while new_x < cols && (engine[new_y as usize][new_x as usize] as char).is_digit(10) {
                num = num * 10
                  + (engine[new_y as usize][new_x as usize] as char)
                    .to_digit(10)
                    .unwrap() as u64;
                engine[new_y as usize][new_x as usize] = 0;
                new_x += 1;
              }
              gear_nums.push(num);
            }
          }
        }

        if gear_nums.len() == 2 {
          sum += gear_nums.iter().fold(1, |acc, x| acc * x);
        }
      }
    }
  }

  println!("{}", sum);

  Ok(())
}
