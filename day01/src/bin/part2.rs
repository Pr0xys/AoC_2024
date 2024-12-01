use std::collections::HashMap;

fn main() {
    let input = include_str!("./input2.txt");
    let (left_list,right_list): (Vec<i32>, Vec<i32>) = input
        .lines()
        .filter_map(|line| {
            let mut line_parts = line.split_whitespace();
            let left: i32 = line_parts.next()?.parse().ok()?;
            let right: i32 = line_parts.next()?.parse().ok()?;
            Some((left, right))
        })
        .unzip();
   
	let mut right_counts:HashMap<i32,i32> = HashMap::new();    
	for num in &right_list {
        *right_counts.entry(*num).or_insert(0) += 1;
    }

	let mut total_output:i32 = 0;
    for num in &left_list {
        if let Some(&count) = right_counts.get(num) {
            total_output += count * num;
        }
    }
	
	println!("{}", total_output);
}
