use std::collections::HashMap;

fn main() {
    let input_as_str = include_str!("input2.txt");

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = vec![];

    for line in input_as_str.lines() {
        match line {
            "" => continue,

            line if line.contains('|') => {
                let parts: Vec<&str> = line.split('|').collect();
                match (parts.get(0), parts.get(1)) {
                    (Some(left), Some(right)) => {
                        match (left.trim().parse::<i32>(), right.trim().parse::<i32>()) {
                            (Ok(left_num), Ok(right_num)) => {
                                rules
                                    .entry(left_num)
                                    .or_insert_with(Vec::new)
                                    .push(right_num);
                            }
                            _ => println!(
                                "Failed to parse number for tuple for rules vec, check it out: {}",
                                line
                            ),
                        }
                    }
                    _ => println!("Weird tuple, check it out: {}", line),
                }
            }

            _ => {
                let parts: Result<Vec<i32>, _> = line
                    .trim()
                    .split(',')
                    .map(|s| s.trim().parse::<i32>())
                    .collect();

                match parts {
                    Ok(numbers) => updates.push(numbers),

                    Err(_) => println!("Failed to parse update vector, check it out: {}", line),
                }
            }
        }
    }

    println!("Rules: {:?}", rules);
    println!("Updates: {:?}", updates);

    let mut output: i32 = 0;

    for update in updates.iter() {
        let mut is_valid = true;
        let mut to_be_sorted: Vec<i32> = vec![];

        'outer: for (i, &current) in update.iter().enumerate() {
            for &next in update.iter().skip(i + 1) {
                if let Some(dependents) = rules.get(&current) {
                    if dependents.contains(&next) {
                        continue;
                    }
                }
                if let Some(dependents) = rules.get(&next) {
                    if dependents.contains(&current) {
                        is_valid = false;
                        let mut aux_vec = vec![];
                        for &number in update.iter() {
                            if aux_vec.is_empty() {
                                aux_vec.push(number);
                            } else {
                                let mut inserted = false;
                                for (i, &existing) in aux_vec.iter().enumerate() {
                                    if let Some(dependents) = rules.get(&number) {
                                        if dependents.contains(&existing) {
                                            aux_vec.insert(i, number);
                                            inserted = true;
                                            break;
                                        }
                                    }
                                    if let Some(dependents) = rules.get(&existing) {
                                        if dependents.contains(&number) {
                                            continue;
                                        }
                                    }
                                }
                                if !inserted {
                                    aux_vec.push(number);
                                }
                            }
                        }

                        to_be_sorted = aux_vec.clone();
                        break 'outer;
                    }
                }
            }
        }

        if !is_valid {
            println!("Invalid: {:?}, now valid: {:?}", update, to_be_sorted);

            if !to_be_sorted.is_empty() {
                let middle_index = update.len() / 2;
                output += to_be_sorted[middle_index];
            }
        }
    }

    println!("Total: {}", output);
}
