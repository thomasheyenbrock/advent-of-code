const INPUT: &str = include_str!("input.txt");

fn main() {
    let tiles = INPUT
        .lines()
        .map(|line| {
            let mut iter = line.split(",");
            (
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut max_x = 0;
    let mut max_y = 0;
    for (x, y) in tiles.iter() {
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
    }
    max_x += 2;
    max_y += 2;

    let mut grid = vec![vec![false; max_y]; max_x];
    for i in 0..tiles.len() {
        let (x1, y1) = tiles[i];
        let (x2, y2) = tiles[(i + 1) % tiles.len()];

        if x1 == x2 && y1 < y2 {
            let mut z = y1;
            while z != y2 {
                grid[x1][z] = true;
                z += 1;
            }
        } else if x1 == x2 && y1 > y2 {
            let mut z = y1;
            while z != y2 {
                grid[x1][z] = true;
                z -= 1;
            }
        } else if y1 == y2 && x1 < x2 {
            let mut z = x1;
            while z != x2 {
                grid[z][y1] = true;
                z += 1;
            }
        } else if y1 == y2 && x1 > x2 {
            let mut z = x1;
            while z != x2 {
                grid[z][y1] = true;
                z -= 1;
            }
        } else {
            unreachable!();
        }
    }

    let border_grid = grid.clone();

    for row in 0..grid.len() {
        let mut is_inside = false;
        let mut col = 0;
        while col < max_y {
            if !border_grid[row][col] {
                // Not a border cell, color it if we're inside, else do nothing
                if is_inside {
                    grid[row][col] = true;
                }
                col += 1;
                continue;
            }

            if col + 1 == max_y {
                // Border cell at the end of the grid, do nothing
                col += 1;
                continue;
            }

            if !border_grid[row][col + 1] {
                // Next cell not a border, switch inside flag and continue
                is_inside = !is_inside;
                col += 1;
                continue;
            }

            // Horizontal border strip case
            let is_from_above_start = row > 0 && border_grid[row - 1][col];
            let is_from_below_start = row + 1 < max_x && border_grid[row + 1][col];
            if !(is_from_above_start ^ is_from_below_start) {
                unreachable!();
            }

            while col + 1 < max_y && border_grid[row][col + 1] {
                col += 1;
            }

            let is_from_above_end = row > 0 && border_grid[row - 1][col];
            let is_from_below_end = row + 1 < max_x && border_grid[row + 1][col];
            if !(is_from_above_end ^ is_from_below_end) {
                unreachable!();
            }

            if is_from_above_start ^ is_from_above_end {
                is_inside = !is_inside;
            }
            col += 1;
        }
    }

    let mut tile_pairs = Vec::with_capacity(tiles.len() * (tiles.len() - 1) / 2);
    for a in 0..tiles.len() - 1 {
        for b in a + 1..tiles.len() {
            let a = &tiles[a];
            let b = &tiles[b];

            let min = (a.0.min(b.0), a.1.min(b.1));
            let max = (a.0.max(b.0), a.1.max(b.1));

            let area = (max.0 - min.0 + 1) * (max.1 - min.1 + 1);

            tile_pairs.push((min, max, area));
        }
    }

    tile_pairs.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    let max_area = tile_pairs[0].2;
    println!("{max_area}");

    let mut corners = Vec::with_capacity(tiles.len());
    for tile in tiles {
        if tile.0 > 0 {
            if !grid[tile.0 - 1][tile.1] {
                corners.push(tile);
                continue;
            }
            if tile.1 > 0 && !grid[tile.0 - 1][tile.1 - 1] {
                corners.push(tile);
                continue;
            }
            if tile.1 + 1 < max_y && !grid[tile.0 - 1][tile.1 + 1] {
                corners.push(tile);
                continue;
            }
        }

        if tile.0 + 1 < max_x {
            if !grid[tile.0 + 1][tile.1] {
                corners.push(tile);
                continue;
            }
            if tile.1 > 0 && !grid[tile.0 + 1][tile.1 - 1] {
                corners.push(tile);
                continue;
            }
            if tile.1 + 1 < max_y && !grid[tile.0 + 1][tile.1 + 1] {
                corners.push(tile);
                continue;
            }
        }

        if tile.1 > 0 && !grid[tile.0][tile.1 - 1] {
            corners.push(tile);
            continue;
        }

        if tile.1 + 1 < max_y && !grid[tile.0][tile.1 + 1] {
            corners.push(tile);
            continue;
        }
    }

    'tile_loop: for (min, max, area) in tile_pairs.iter() {
        // Optimization: If a corner is strictly inside the rect, then it cannot be fully enclosed
        for c in corners.iter() {
            if min.0 < c.0 && c.0 < max.0 && min.1 < c.1 && c.1 < max.1 {
                continue 'tile_loop;
            }
        }

        for row in min.0..max.0 {
            for col in min.1..max.1 {
                if !grid[row][col] {
                    continue 'tile_loop;
                }
            }
        }

        println!("{area}");
        break;
    }
}
