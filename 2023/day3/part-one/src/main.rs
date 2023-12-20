use std::io::{self, Result};

fn main() -> Result<()> {
  let mut engine = [[0u8; 1000]; 1000];
  let mut engine_mark = [[false; 1000]; 1000];
  let mut cols: i32 = 0;
  let mut rows: i32 = 0;
  for (i, line) in io::stdin().lines().enumerate() {
    if let Ok(line) = line {
      for (j, c) in line.as_bytes().into_iter().enumerate() {
        engine[i][j] = *c;
      }
      cols = line.len() as i32;
    }
    rows += 1;
  }
  for i in 0..rows {
    for j in 0..cols {
      let c = engine[i as usize][j as usize];
      if !c.is_ascii_digit() && c != '.' as u8 {
        for y in -1..2 {
          for x in -1..2 {
            if x == 0 && y == 0 {
              continue;
            }
            let new_x: i32 = j as i32 + x;
            let new_y: i32 = i as i32 + y;
            if new_x >= 0 && new_x < cols && new_y >= 0 && new_y < rows {
              engine_mark[new_y as usize][new_x as usize] = true;
            }
          }
        }
      }
    }
  }

  let mut sum: u32 = 0;

  for i in 0..rows {
    let mut num = 0;
    let mut is_part = false;
    for j in 0..cols {
      let c = engine[i as usize][j as usize];
      if (c as char).is_digit(10) {
        num = num * 10 + (c as char).to_digit(10).unwrap();
        if engine_mark[i as usize][j as usize] {
          is_part = true;
        }
      } else {
        if num > 0 && is_part {
          sum += num;
        }
        num = 0;
        is_part = false;
      }
    }

    if num > 0 && is_part {
      sum += num;
    }
  }

  println!("{}", sum);

  Ok(())
}
