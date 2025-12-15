const INPUT: &str = include_str!("input.txt");

fn main() {
    let grid = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let rows = grid.len();
    let cols = grid[0].len();

    // PART 1
    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] != 'X' {
                continue;
            }

            if i + 3 < rows
                && grid[i + 1][j] == 'M'
                && grid[i + 2][j] == 'A'
                && grid[i + 3][j] == 'S'
            {
                count += 1
            }

            if i >= 3 && grid[i - 1][j] == 'M' && grid[i - 2][j] == 'A' && grid[i - 3][j] == 'S' {
                count += 1
            }

            if j + 3 < cols
                && grid[i][j + 1] == 'M'
                && grid[i][j + 2] == 'A'
                && grid[i][j + 3] == 'S'
            {
                count += 1
            }

            if j >= 3 && grid[i][j - 1] == 'M' && grid[i][j - 2] == 'A' && grid[i][j - 3] == 'S' {
                count += 1
            }

            if i + 3 < rows
                && j + 3 < cols
                && grid[i + 1][j + 1] == 'M'
                && grid[i + 2][j + 2] == 'A'
                && grid[i + 3][j + 3] == 'S'
            {
                count += 1
            }

            if i + 3 < rows
                && j >= 3
                && grid[i + 1][j - 1] == 'M'
                && grid[i + 2][j - 2] == 'A'
                && grid[i + 3][j - 3] == 'S'
            {
                count += 1
            }

            if i >= 3
                && j + 3 < cols
                && grid[i - 1][j + 1] == 'M'
                && grid[i - 2][j + 2] == 'A'
                && grid[i - 3][j + 3] == 'S'
            {
                count += 1
            }

            if i >= 3
                && j >= 3
                && grid[i - 1][j - 1] == 'M'
                && grid[i - 2][j - 2] == 'A'
                && grid[i - 3][j - 3] == 'S'
            {
                count += 1
            }
        }
    }

    println!("{count}");

    // PART 2
    let mut count = 0;

    for i in 1..(rows - 1) {
        for j in 1..(cols - 1) {
            if grid[i][j] != 'A' {
                continue;
            }

            let top_down_is_ms = grid[i - 1][j - 1] == 'M' && grid[i + 1][j + 1] == 'S';
            let top_down_is_sm = grid[i - 1][j - 1] == 'S' && grid[i + 1][j + 1] == 'M';
            if !(top_down_is_ms || top_down_is_sm) {
                continue;
            }

            let bottom_up_is_ms = grid[i + 1][j - 1] == 'M' && grid[i - 1][j + 1] == 'S';
            let bottom_up_is_sm = grid[i + 1][j - 1] == 'S' && grid[i - 1][j + 1] == 'M';
            if !(bottom_up_is_ms || bottom_up_is_sm) {
                continue;
            }

            count += 1
        }
    }

    println!("{count}");
}
