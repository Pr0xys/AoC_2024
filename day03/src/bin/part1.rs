use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    
    // I am not Regex pro :D this regex was made by brute force on regex101
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut output: i32 = 0;

    for captured in regex.captures_iter(input) {
        let digit1 = captured.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let digit2 = captured.get(2).unwrap().as_str().parse::<i32>().unwrap();
        output += digit1 * digit2
    }   
       
    println!("Output: {:?}", output);
}
