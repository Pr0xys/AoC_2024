fn main() {
    let input_as_str = include_str!("input1.txt");
    let grid: Vec<Vec<char>> = input_as_str
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let directions: [&[(isize, isize)]; 4] = [
        &[(0, 0), (1, -1), (2, -2), (3, -3)], // UP-RIGHT
        &[(0, 0), (1, 0), (2, 0), (3, 0)],   // RIGHT
        &[(0, 0), (1, 1), (2, 2), (3, 3)],   // DOWN-RIGHT
        &[(0, 0), (0, 1), (0, 2), (0, 3)],   // DOWN
    ];

    let mut output = 0;
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    for row in 0..rows {
        for col in 0..cols {
            for dir in &directions {
                let mut word = Vec::new();
                for &(d_col, d_row) in *dir {
                    let n_col = col + d_col;
                    let n_row = row + d_row;

                    if n_col < 0 || n_row < 0 || n_col >= cols || n_row >= rows {
                        break;
                    }

                    word.push(grid[n_row as usize][n_col as usize]);
                }

                if word.len() == 4 && (word.iter().collect::<String>() == "XMAS" || word.iter().collect::<String>() == "SAMX") {
                    output += 1;
                }
            }
        }
    }

    println!("{}", output);
}

