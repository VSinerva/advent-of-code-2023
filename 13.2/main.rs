use std::io;

fn main()
{
    let input = io::stdin().lines();

    let mut total = 0_usize;
    let mut grid: Vec<Vec<i32>> = Vec::new();

    for line in input {
        let string = line.unwrap();

        if string.len() == 0 && grid.len() > 0 {
            let mut done = false;
            let mut result;
            let prev_result = find_lines(&grid, (0, 0));
            for row_i in 0..grid.len() {
                for col_i in 0..grid[0].len() {
                    if grid[row_i][col_i] == 1 {
                        grid[row_i][col_i] = 0;
                        result = find_lines(&grid, prev_result);
                        grid[row_i][col_i] = 1;
                    } else {
                        grid[row_i][col_i] = 1;
                        result = find_lines(&grid, prev_result);
                        grid[row_i][col_i] = 0;
                    }
                    if prev_result != result  && result != (0,0) {
                        if prev_result.0 != result.0 {
                            total += result.0;
                        }
                        if prev_result.1 != result.1 {
                            total += 100 * result.1;
                        }
                        done = true;
                        break;
                    }
                }
                if done {
                    break;
                }
            }
            grid.clear();
        } else {
            grid.push(string.chars().map(|s| if s == '.' { 0 } else { 1 }).collect::<Vec<i32>>());
        }

        if string.chars().nth(0).unwrap_or('_') == 'Q' {
            break;
        }

    }
    println!("{:?}", total);
}

fn find_lines(grid: &Vec<Vec<i32>>, prev_result: (usize, usize))  -> (usize, usize) {
    let mut possible_values: Vec<usize> = (1..grid[0].len()).collect();
    for row in grid {
        let mut impossible: Vec<usize> = Vec::new();
        for i in &possible_values {
            let mut start = row[0..*i].iter().rev().peekable();
            let mut end = row[*i..row.len()].iter().peekable();

            let mut possible = true;
            while start.peek().is_some() && end.peek().is_some() {
                if start.next().unwrap() != end.next().unwrap() {
                    possible = false;
                    break;
                }
            }
            if !possible {
                impossible.push(*i);
            }
        }
        possible_values.retain(|x| !impossible.contains(x));
    }
    let mut col = 0;
    for x in &possible_values {
        if *x != prev_result.0 {
            col = *x;
            break;
        }
    }

    possible_values.clear();
    for i in 1..grid.len() {
        let mut start = grid[0..i].iter().rev().peekable();
        let mut end = grid[i..grid.len()].iter().peekable();

        let mut possible = true;
        while start.peek().is_some() && end.peek().is_some() {
            if start.next().unwrap() != end.next().unwrap() {
                possible = false;
                break;
            }
        }

        if possible{
            possible_values.push(i);
        }
    }
    let mut row = 0;
    for x in &possible_values {
        if *x != prev_result.1 {
            row = *x;
            break;
        }
    }

    return (col, row);
}
