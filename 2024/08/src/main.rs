use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

const CHARS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

fn main() {
    let grid = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let rows = grid.len();
    let cols = grid[0].len();

    // PART 1
    let mut antinodes = HashSet::<(usize, usize)>::new();

    for char in CHARS.chars() {
        let mut positions = vec![];
        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == char {
                    positions.push((i, j));
                }
            }
        }

        if positions.is_empty() {
            continue;
        }

        for m in 0..(positions.len() - 1) {
            for n in (m + 1)..positions.len() {
                let (ax, ay) = positions[m];
                let (bx, by) = positions[n];

                if bx + bx >= ax && bx + bx < ax + rows && by + by >= ay && by + by < ay + cols {
                    antinodes.insert((bx + bx - ax, by + by - ay));
                }
                if ax + ax >= bx && ax + ax < bx + rows && ay + ay >= by && ay + ay < by + cols {
                    antinodes.insert((ax + ax - bx, ay + ay - by));
                }
            }
        }
    }

    println!("{}", antinodes.len());

    // PART 2
    let mut antinodes = HashSet::<(usize, usize)>::new();

    for char in CHARS.chars() {
        let mut positions = vec![];
        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == char {
                    positions.push((i, j));
                }
            }
        }

        if positions.is_empty() {
            continue;
        }

        for m in 0..(positions.len() - 1) {
            for n in (m + 1)..positions.len() {
                let (ax, ay) = positions[m];
                let (bx, by) = positions[n];

                let mut cx = ax;
                let mut cy = ay;
                antinodes.insert((cx, cy));

                while cx + ax >= bx && cx + ax < bx + rows && cy + ay >= by && cy + ay < by + cols {
                    cx = cx + ax - bx;
                    cy = cy + ay - by;
                    antinodes.insert((cx, cy));
                }

                let mut cx = bx;
                let mut cy = by;
                antinodes.insert((cx, cy));

                while cx + bx >= ax && cx + bx < ax + rows && cy + by >= ay && cy + by < ay + cols {
                    cx = cx + bx - ax;
                    cy = cy + by - ay;
                    antinodes.insert((cx, cy));
                }
            }
        }
    }

    println!("{}", antinodes.len());
}
