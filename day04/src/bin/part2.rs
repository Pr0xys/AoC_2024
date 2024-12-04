fn main() {
    let input_as_str = include_str!("input2.txt");
    let grid: Vec<Vec<char>> = input_as_str
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut output = 0;
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    for row in 0..rows {
        for col in 0..cols {
            let directions = [
                (col + 1, row + 1), // CENTER
                (col, row),         // UP-RIGHT
                (col, row + 2),     // DOWN-RIGHT
                (col + 2, row),     // UP-LEFT
                (col + 2, row + 2), // DOWN-LEFT
            ];

            let mut word = Vec::new();
            let mut valid = true;

            for &(n_col, n_row) in &directions {
                if n_col < 0 || n_row < 0 || n_col >= cols || n_row >= rows {
                    valid = false;
                    break;
                }

                word.push(grid[n_row as usize][n_col as usize]);
            }

            if valid && word.len() == 5 {
                if word[0] != 'A' {
                    continue;
                }

                let pattern = &word[1..];
                if pattern == &['M', 'M', 'S', 'S']
                    || pattern == &['M', 'S', 'M', 'S']
                    || pattern == &['S', 'S', 'M', 'M']
                    || pattern == &['S', 'M', 'S', 'M']
                {
                    output += 1;
                }
            }
        }
    }

    println!("{}", output);
}
