#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Dot,
    Digit,
    Symbol,
}

fn main() {
    println!(
        "{}",
        solve(std::fs::read_to_string("./input").expect("input to read"))
    )
}

fn cell(c: char) -> Cell {
    if c == '.' || c == '\n' {
        Cell::Dot
    } else if c.is_digit(10) {
        Cell::Digit
    } else {
        Cell::Symbol
    }
}

#[rustfmt::skip]
fn solve(data: String) -> i32 {
    let width = (data.find("\n").unwrap() + 1) as i32;
    let chars = data.chars().collect::<Vec<char>>();

    let offsets = [
        -1, -width - 1, -width, -width + 1,
         1,  width - 1,  width,  width + 1,
    ];

    chars
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| {
            if cell(c) != Cell::Symbol {
                return None;
            }

            let mut nums = offsets
                .iter()
                .filter_map(|offset| {
                    chars
                        .get((i as i32 + offset) as usize)
                        .map(|c| ((i as i32 + offset) as usize, c))
                })
                .filter_map(|(ni, &nc)| {
                    if cell(nc) != Cell::Digit {
                        return None;
                    }

                    let left_bound = &chars[..ni]
                        .iter()
                        .rev()
                        .take_while(|&&c| c.is_digit(10))
                        .count();

                    let right_bound = &chars[ni..]
                        .iter()
                        .take_while(|&&c| c.is_digit(10))
                        .count();

                    let (start, end) = (ni - left_bound, ni + right_bound);
                    let number = chars[start..end]
                        .iter()
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap();

                    return Some((number, start, end));
                })
                .collect::<Vec<(i32, usize, usize)>>();

            nums.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
            nums.dedup();

            Some(nums.iter().fold(0, |acc, num| acc + num.0))
        })
        .sum::<i32>()
}
