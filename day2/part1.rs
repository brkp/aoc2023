fn main() {
    println!(
        "{}",
        solve(std::fs::read_to_string("./input").expect("input to read"))
    )
}

fn solve(data: String) -> i32 {
    data.lines()
        .filter_map(|line| {
            let (game, sets) = line.split_once(": ").unwrap();

            let game = game.trim_start_matches("Game ").parse::<i32>().unwrap();
            let sets = sets.split("; ").map(|set| {
                let (mut r, mut g, mut b) = (0, 0, 0);

                #[rustfmt::skip]
                set.split(", ").for_each(|pick| {
                    let (n, color) = pick.split_once(" ").unwrap();
                    let n = n.parse::<i32>().unwrap();

                    match color {
                        "red"   => r = n,
                        "green" => g = n,
                        "blue"  => b = n,
                        _ => unreachable!(),
                    }
                });

                (r, g, b)
            });

            // why does `sets.any` take `&mut self`??
            let mut result = None;
            for set in sets {
                if !(set.0 > 12 || set.1 > 13 || set.2 > 14) {
                    result = Some(game);
                    break;
                }
            }
            result
        })
        .sum()
}
