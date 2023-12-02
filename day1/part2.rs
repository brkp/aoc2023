use std::collections::HashMap;

fn main() {
    println!(
        "{}",
        solve(std::fs::read_to_string("./input").expect("input to read"))
    )
}

fn solve(data: String) -> i32 {
    #[rustfmt::skip]
    let map = [
        ("1", 1), ("2", 2), ("3", 3),
        ("4", 4), ("5", 5), ("6", 6),
        ("7", 7), ("8", 8), ("9", 9),
        ("one", 1), ("two", 2), ("three", 3),
        ("four", 4), ("five", 5), ("six", 6),
        ("seven", 7), ("eight", 8), ("nine", 9),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<&str, i32>>();

    data.lines()
        .map(|line| {
            let mut nums = vec![];

            'first: for i in 0..line.len() {
                for (k, v) in map.iter() {
                    if i + k.len() > line.len() {
                        continue;
                    }

                    if line[i..i + k.len()] == **k {
                        nums.push(*v);
                        break 'first;
                    }
                }
            }

            'second: for i in (0..line.len()).rev() {
                for (k, v) in map.iter() {
                    if i + k.len() > line.len() {
                        continue;
                    }

                    if line[i..i + k.len()] == **k {
                        nums.push(*v);
                        break 'second;
                    }
                }
            }

            nums[0] * 10 + nums[1]
        })
        .sum::<i32>()
}
