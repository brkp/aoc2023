fn main() {
    println!(
        "{}",
        solve(std::fs::read_to_string("./input").expect("input to read"))
    )
}

fn solve(data: String) -> u32 {
    data.lines()
        .map(|line| {
            let mut nums = vec![];

            for c in line.chars() {
                if let Some(d) = c.to_digit(10) {
                    nums.push(d);
                    break;
                }
            }

            for c in line.chars().rev() {
                if let Some(d) = c.to_digit(10) {
                    nums.push(d);
                    break;
                }
            }

            nums[0] * 10 + nums[1]
        })
        .sum::<u32>()
}
