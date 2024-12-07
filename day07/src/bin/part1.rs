fn main() {
    let input = include_str!("./input1.txt");

    let input: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.split(|c: char| c == ' ' || c == ':')
                .filter(|x| !x.is_empty())
                .filter_map(|x| x.parse::<i64>().ok())
                .collect()
        })
        .collect();

    let mut output: i64 = 0;
    
    fn calculate(operands: &[i64], index: usize, current_result: i64, target: i64, found: &mut bool) {
        if *found {
            return;
        }

        if index == operands.len() {
            if current_result == target {
                *found = true;
            }
            return;
        }

        calculate(operands, index + 1, current_result + operands[index], target, found);
        calculate(operands, index + 1, current_result * operands[index], target, found);
    }

    for equations in input.iter() {
        let target = equations[0];
        let operands = &equations[1..];

        if target == operands.iter().sum::<i64>() {
            output += target;
            continue;
        }

        let mut found = false;

        calculate(operands, 1, operands[0], target, &mut found);

        if found {
            output += target;
        } else {
            println!("Not valid {}", target);
        }
    }

    println!("Final Output: {}", output);
}
