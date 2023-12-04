use std::{io, collections::HashSet};

fn main()
{
    let data = io::stdin().lines();
    let mut sum = 0;

    for line in data {
        let string = line.unwrap();
        if string.len() == 0 {
            break;
        }

        let cards = string.split(":").last().unwrap();

        let mut temp = cards.split("|");
        let winning_numbers: HashSet<i32> = temp.next().unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        let mut your_numbers: Vec<i32> = temp.next().unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        your_numbers.retain(|x| winning_numbers.contains(x));
        let winning_count = your_numbers.len() as i32;

        if winning_count > 0 {
            sum += 1 << (winning_count - 1);
        }
    }

    println!("{}", sum);
}
