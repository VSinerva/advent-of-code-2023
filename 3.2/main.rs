use std::io;

fn main()
{
    let data = io::stdin().lines();
    let mut sum = 0;

    let mut string_vector = Vec::new();

    for line in data {
        let string = line.unwrap();
        if string.len() == 0 {
            break;
        }

        string_vector.push(string);
    }

    let mut numbers = Vec::new();
    for string in string_vector.clone() {
        let num_strs: Vec<&str> = string.split(|c: char| !c.is_digit(10)).collect();
        for num_str in num_strs{
            let num_string = num_str.to_string();
            if num_string != "" {
                numbers.push(num_string.parse::<i32>().unwrap());
            }
        }
    }

    let mut number_indices = Vec::new();
    let mut next_index = 0;

    for string in string_vector.clone() {
        let mut num_started = false;
        let mut index_row: Vec<i32> = Vec::new();

        for character in string.chars() {
            if character.is_digit(10) {
                num_started = true;
                index_row.push(next_index);
            } else {
                if num_started {
                    num_started = false;
                    next_index += 1;
                }
                index_row.push(-1);
            }
        }

        if num_started {
            next_index += 1;
        }

        number_indices.push(index_row.clone());
        index_row.clear();
    }


    let max_row = number_indices.len() as i32;
    let max_col = number_indices[0].len() as i32;
    for (row_i, string) in string_vector.into_iter().enumerate() {
        for (col_i, character) in string.chars().into_iter().enumerate() {
            if character == '*' {
                let mut adjacent = Vec::new();
                for d_row in -1..=1 {
                    for d_col in -1..=1 {
                        let row = row_i as i32 + d_row;
                        let col = col_i as i32 + d_col;
                        if row > -1 && row < max_row && col > -1 && col < max_col {
                            let index = number_indices[row as usize][col as usize];
                            if index > -1 {
                                adjacent.push(index);
                            }
                        }
                    }
                }
                adjacent.dedup();

                if adjacent.len() == 2 {
                    sum += numbers[adjacent[0] as usize] * numbers[adjacent[1] as usize];
                }
            }
        }
    }

    println!("{}", sum);
}
