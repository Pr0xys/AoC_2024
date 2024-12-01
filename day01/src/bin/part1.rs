fn main() {
    let input = include_str!("./input1.txt");

    let (mut left_list, mut right_list): (Vec<i32>, Vec<i32>) = input
        .lines()
        .filter_map(|line| {
            let mut line_parts = line.split_whitespace();
            let left: i32 = line_parts.next()?.parse().ok()?;
            let right: i32 = line_parts.next()?.parse().ok()?;
            Some((left, right))
        })
        .unzip();

    left_list.sort();
    right_list.sort();

    let output: i32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, right)| (left - right).abs())
        .sum();

    println!("{}", output);
}
