fn main() {
    let input_as_str = include_str!("input1.txt");

    let mut grid: Vec<Vec<char>> = input_as_str
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // UP, RIGHT, DOWN, LEFT

    let mut start_row = 0;
    let mut start_col = 0;
    let mut start_dir = '^';

    'outer: for (r, row) in grid.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if "^>v<".contains(cell) {
                start_row = r;
                start_col = c;
                start_dir = cell;
                break 'outer;
            }
        }
    }

    let mut dir_index = match start_dir {
        '^' => 0, // UP
        '>' => 1, // RIGHT
        'v' => 2, // DOWN
        '<' => 3, // LEFT
        _ => panic!("Invalid starting direction!"),
    };

    grid[start_row][start_col] = 'X';
    let mut row = start_row;
    let mut col = start_col;

    loop {
        if grid[row][col] == '.' {
            grid[row][col] = 'X';
        }

        let next_row = row as isize + directions[dir_index].0;
        let next_col = col as isize + directions[dir_index].1;

        if next_row < 0
            || next_row >= grid.len() as isize
            || next_col < 0
            || next_col >= grid[0].len() as isize
        {
            break;
        }

        if grid[next_row as usize][next_col as usize] == '#' {
            dir_index = (dir_index + 1) % 4;
        } else {
            row = next_row as usize;
            col = next_col as usize;
        }
    }

    for i in grid.iter() {
        println!("{:?}", i);
    }

    println!(
        "Output: {}",
        grid.iter()
            .flat_map(|row| row.iter())
            .filter(|&&cell| cell == 'X')
            .count()
    );
}
