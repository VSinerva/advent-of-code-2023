use std::{io, cmp::{max, min}};

fn main()
{
    let input = io::stdin().lines();

    let mut char_grid: Vec<Vec<char>> = Vec::new();

    for line in input {
        let string = line.unwrap();
        if string.len() == 0 {
            break;
        }

        char_grid.push(string.chars().collect::<Vec<char>>());
    }

    let mut points: Vec<(usize, usize)> = Vec::new();
    for (row_i, line) in char_grid.iter().enumerate() {
        for (col_i, c) in line.iter().enumerate() {
            if *c == '#' {
                points.push((row_i, col_i));
            }
        }
    }

    for i in (0..char_grid[0].len()).rev() {
        let mut empty = true;
        for line in &char_grid {
            if line[i] == '#' {
                empty = false;
                break;
            }
        }

        if empty {
            for point in &mut points {
                if point.1 > i {
                    point.1 += 1;
                }
            }
        }
    }

    for i in (0..char_grid.len()).rev() {
        if !char_grid[i].contains(&'#') {
            for point in &mut points {
                if point.0 > i {
                    point.0 += 1;
                }
            }
        }
    }

    let mut total = 0_i128;
    for (i, a) in points.iter().enumerate() {
        for j in i+1..points.len() {
            total += manhattan(a, &points[j]) as i128;
        }
    }
    println!("{:?}", total);
}

fn manhattan(a: &(usize, usize), b: &(usize, usize)) -> i32 {
    let (ax, ay) = a;
    let (bx, by) = b;

    ((max(ax, bx) - min(ax, bx)) + (max(ay, by) - min(ay, by))) as i32
}
