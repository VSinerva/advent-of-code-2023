use std::{io, collections::HashSet};

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

    let mut connections: HashSet<(i32, i32)> = HashSet::new();
    let mut id = 0;
    let mut start_id = 0;
    let row_len = char_grid[0].len() as i32;
    let row_count = char_grid.len() as i32;
    for (row_i, row) in char_grid.iter().enumerate() {
        for (c_i, c) in row.iter().enumerate() {
            let row_i32 = row_i as i32;
            let col_i32 = c_i as i32;
            if *c == 'S' {
                start_id = id;
            } else if *c == '|' {
                if row_i32 > 0 {
                    connections.insert((id, id - row_len as i32));
                }
                if row_i32 < row_len-1 {
                    connections.insert((id, id + row_len));
                }
            } else if *c == '-' {
                if col_i32 > 0 {
                    connections.insert((id, id - 1));
                }
                if col_i32 < row_len-1 {
                    connections.insert((id, id + 1));
                }
            } else if *c == 'L' {
                if row_i32 > 0 {
                    connections.insert((id, id - row_len));
                }
                if col_i32 < row_len-1 {
                    connections.insert((id, id + 1));
                }
            } else if *c == 'J' {
                if row_i32 > 0 {
                    connections.insert((id, id - row_len));
                }
                if col_i32 > 0 {
                    connections.insert((id, id - 1));
                }
            } else if *c == '7' {
                if col_i32 > 0 {
                    connections.insert((id, id - 1));
                }
                if row_i32 < row_count-1 {
                    connections.insert((id, id + row_len));
                }
            } else if *c == 'F' {
                if row_i32 < row_count-1 {
                    connections.insert((id, id + row_len));
                }
                if col_i32 < row_len-1 {
                    connections.insert((id, id + 1));
                }
            }
            id += 1;
        }
    }

    if connections.contains(&(start_id-row_len, start_id)) {
        connections.insert((start_id, start_id-row_len));
    }
    if connections.contains(&(start_id+row_len, start_id)) {
        connections.insert((start_id, start_id+row_len));
    }
    if connections.contains(&(start_id-1, start_id)) {
        connections.insert((start_id, start_id-1));
    }
    if connections.contains(&(start_id+1, start_id)) {
        connections.insert((start_id, start_id+1));
    }

    id = start_id;
    let mut prev = -1;
    let mut steps = 0;
    while steps < 1 || id != start_id {
        let mut new_id = -1;
        if prev != id-row_len && connections.contains(&(id, id-row_len)) {
            new_id = id - row_len;
        } else if prev != id+row_len && connections.contains(&(id, id+row_len)) {
            new_id = id + row_len;
        } else if prev != id-1 && connections.contains(&(id, id-1)) {
            new_id = id - 1;
        } else if prev != id+1 && connections.contains(&(id, id+1)) {
            new_id = id + 1;
        }

        println!("{:?} {:?}", 1 + (id / row_len), 1 + (id % row_len));
        steps += 1;
        prev = id;
        id = new_id;
    }

    println!("{:?}", steps / 2);
}
