use std::io;

fn main()
{
    let input = io::stdin().lines();

    let mut total = 0;
    for line in input {
        let string = line.unwrap();
        if string.len() == 0 {
            break;
        }
        let parts: Vec<String> = string.split_whitespace().map(|s| s.to_string()).collect();

        let mut damaged_vec: Vec<bool> = Vec::new();
        let damaged_string = parts[0].clone();
        let groups: Vec<i32> = parts[1].split(',').map(|x| x.parse::<i32>().unwrap() ).collect();

        total += valid_permutation_count(&mut damaged_vec, &damaged_string, &groups, 0);
    }
    println!("{:?}", total);
}

fn valid_permutation_count(damaged_vec: &mut Vec<bool>, string: &String, groups: &Vec<i32>, index: usize)  -> i32 {
    if damaged_vec.len() == string.len() {
        let mut final_groups: Vec<i32> = Vec::new();
        let mut current_group = 0;
        for damaged in damaged_vec {
            if *damaged {
                current_group += 1;
            } else if current_group > 0 {
                final_groups.push(current_group);
                current_group = 0;
            }
        }
        if current_group > 0 {
            final_groups.push(current_group);
            current_group = 0;
        }

        if &final_groups == groups {
            return 1;
        } else {
            return 0;
        }
    } else {
        let mut sum = 0;
        if string.chars().nth(index) == Some('#') {
            damaged_vec.push(true);
            sum += valid_permutation_count(damaged_vec, string, groups, index + 1);
            damaged_vec.pop();
        } else if string.chars().nth(index) == Some('.') {
            damaged_vec.push(false);
            sum += valid_permutation_count(damaged_vec, string, groups, index + 1);
            damaged_vec.pop();
        } else if string.chars().nth(index) == Some('?') {
            damaged_vec.push(true);
            sum += valid_permutation_count(damaged_vec, string, groups, index + 1);
            damaged_vec.pop();
            damaged_vec.push(false);
            sum += valid_permutation_count(damaged_vec, string, groups, index + 1);
            damaged_vec.pop();
        }

        return sum;
    }
}
