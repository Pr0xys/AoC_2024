fn main() {
    let input = include_str!("./inputtest.txt");

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
            //println!("This was SAFE: {:?}", levels);
            output += 1;
        }
    }

    println!("{}", output);
}
