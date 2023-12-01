use std::io;

fn main()
{
    let mut sum = 0;
    let data = io::stdin().lines();

    for line in data {
        let string = line.unwrap();

        if string.len() == 0 {
            break;
        }

        for character in string.chars() {
            if let Some(val) = character.to_digit(10) {
                sum += val * 10;
                break;
            }
        }

        for character in string.chars().rev() {
            if let Some(val) = character.to_digit(10) {
                sum += val;
                break;
            }
        }
    }

    println!("{}", sum);
}