fn main() {
    let input = include_str!("./input1.txt");

    let all_reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|nums| nums.parse::<i32>().expect("invald num"))
                .collect()
        })
        .collect();

    let mut output: i32 = 0;

    for levels in all_reports {
        let is_increasing: bool = levels.windows(2).all(|nums| {
            let diff: i32 = nums[0] - nums[1];
            (diff == 1 || diff == 2 || diff == 3)  && diff != 0
        });

        let is_decreasing: bool = levels.windows(2).all(|nums| {
            let diff: i32 = nums[0] - nums[1];
            (diff == -3 || diff == -2 || diff == -1) && diff != 0
        });

        if is_increasing || is_decreasing {
            println!("This was SAFE: {:?}", levels);
            output += 1;
            continue;
        }

        let current_levels: Vec<i32> = levels;

        for (idx, &level) in current_levels.iter().enumerate() {
            let modified_level: Vec<i32> = current_levels
                .iter()
                .enumerate()
                .filter_map(|(i, &val)| if i != idx { Some(val) } else { None })
                .collect();

            let is_increasing = modified_level.windows(2).all(|nums| {
                let diff: i32 = nums[0] - nums[1];
                (diff == 1 || diff == 2 || diff == 3) && diff != 0
            });

            let is_decreasing = modified_level.windows(2).all(|nums| {
                let diff: i32 = nums[0] - nums[1];
                (diff == -3 || diff == -2 || diff == -1) && diff != 0
            });

            if is_increasing || is_decreasing {
                output += 1;
                println!("This has become SAFE: {:?} by removing: {:?}", &current_levels, level);
                break;
            }
        }
    }
    println!("{}", output);
}