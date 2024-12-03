use regex::Regex;

fn main() {
    let input = include_str!("./input2.txt");
    
    // I am not Regex pro :D this regex was made by brute force on regex101
    let regex = Regex::new(r"(do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))").unwrap();
    
    let mut enabled = true;
    let mut output: i32 = 0;

    for captured in regex.captures_iter(input) {
        if let Some(instruction) = captured.get(1) {
            let instruction = instruction.as_str();
            if instruction == "don't()" {
                enabled = false;
            } else if instruction == "do()" {
                enabled = true; 
            } else if enabled {
                let digit1: i32 = captured.get(2).unwrap().as_str().parse().unwrap();
                let digit2: i32 = captured.get(3).unwrap().as_str().parse().unwrap();
                output += digit1 * digit2;
            }
        }
    }
      
    println!("Output: {:?}", output);
}
