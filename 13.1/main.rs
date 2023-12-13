use std::io;

fn main()
{
    let input = io::stdin().lines();

    let mut total = 0_usize;
    let mut grid: Vec<Vec<i32>> = Vec::new();

    for line in input {
        let string = line.unwrap();

        if string.len() == 0 && grid.len() > 0 {
            let (col, row) = find_lines(&grid);
            total += col + 100 * row;
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

fn find_lines(grid: &Vec<Vec<i32>>)  -> (usize, usize) {
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
    let col = *possible_values.get(0).unwrap_or(&0_usize);

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
            return (col, i);
        }
    }

    return (col, 0);
}
