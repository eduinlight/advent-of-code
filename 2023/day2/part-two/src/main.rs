use std::io;

fn color_to_index(color: &str) -> Option<usize> {
  match color {
    "red" => Some(0),
    "green" => Some(1),
    "blue" => Some(2),
    _ => None,
  }
}

fn main() -> io::Result<()> {
  let mut game_index_sum = 0;
  for line in io::stdin().lines() {
    if let Ok(line) = line {
      let game = line.split(':').map(|x| x.trim()).collect::<Vec<&str>>();
      let sets = game[1].split(";").map(|x| x.trim()).collect::<Vec<&str>>();
      let mut colors_count = vec![0, 0, 0];
      for set in sets {
        let plays = set.split(',').map(|x| x.trim()).collect::<Vec<&str>>();
        for play in plays {
          let play_split = play.split(' ').map(|x| x.trim()).collect::<Vec<&str>>();
          let color_amount = play_split[0].parse::<u32>().unwrap();
          let color = play_split[1];

          let color_index = color_to_index(color).unwrap();

          colors_count[color_index] = std::cmp::max(colors_count[color_index], color_amount);
        }
      }
      game_index_sum += colors_count.into_iter().fold(1, |acc, e| acc * e);
    }
  }

  println!("{}", game_index_sum);

  Ok(())
}
