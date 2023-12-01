use std::io;
use std::convert::TryInto;

fn main()
{
    let nums = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut sum = 0;
    let data = io::stdin().lines();

    for line in data {
        let string = line.unwrap();
        if string.len() == 0 {
            break;
        }

        let mut values = Vec::new();

        for (index, num) in nums.iter().enumerate() {
            let indices = string.match_indices(num);
            for (i, _) in indices {
                values.push((i, index+1));
            }
        }

        for (index, character) in string.char_indices() {
            if let Some(val) = character.to_digit(10) {
                values.push((index, val.try_into().unwrap()));
            }
        }

        values.sort();
        sum += 10 * values.first().unwrap().1;
        sum += values.last().unwrap().1;
    }

    println!("{:}", sum);
}