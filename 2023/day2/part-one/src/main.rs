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
  let colors_count = vec![12, 13, 14];
  for line in io::stdin().lines() {
    if let Ok(line) = line {
      let game = line.split(':').map(|x| x.trim()).collect::<Vec<&str>>();
      let game_index = game[0]
        .split(' ')
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .parse::<u32>()
        .unwrap();

      let sets = game[1].split(";").map(|x| x.trim()).collect::<Vec<&str>>();
      let mut possible = true;
      for set in sets {
        let plays = set.split(',').map(|x| x.trim()).collect::<Vec<&str>>();
        for play in plays {
          let play_split = play.split(' ').map(|x| x.trim()).collect::<Vec<&str>>();
          let color_amount = play_split[0].parse::<u32>().unwrap();
          let color = play_split[1];

          if colors_count[color_to_index(color).unwrap()] < color_amount {
            possible = false;
          }
        }
      }
      if possible {
        game_index_sum += game_index;
      }
    }
  }

  println!("{}", game_index_sum);

  Ok(())
}
