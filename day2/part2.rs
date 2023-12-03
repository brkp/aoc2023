fn main() {
    println!(
        "{}",
        solve(std::fs::read_to_string("./input").expect("input to read"))
    )
}

fn solve(data: String) -> i32 {
    data.lines()
        .map(|line| {
            let (_, sets) = line.split_once(": ").unwrap();
            #[rustfmt::skip]
            let mins = sets
                .split("; ")
                .map(|set| {
                    let (mut r, mut g, mut b) = (0, 0, 0);

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
                })
                .fold((0, 0, 0), |mut acc, set| {
                    if acc.0 < set.0 { acc.0 = set.0; }
                    if acc.1 < set.1 { acc.1 = set.1; }
                    if acc.2 < set.2 { acc.2 = set.2; }

                    acc
                });

            mins
        })
        .fold(0, |acc, min| acc + (min.0 * min.1 * min.2))
}
