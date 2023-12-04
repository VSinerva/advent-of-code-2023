use std::{io, collections::HashSet};

fn main()
{
    let data = io::stdin().lines();

    let mut card_index: usize = 0;
    let mut card_copies = vec![0];

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

        for i in 1..=winning_count as usize {
            while card_copies.len() <= card_index + i {
                card_copies.push(0);
            }
            card_copies[card_index + i] += 1 + card_copies[card_index];
        }

        card_index += 1;
    }

    card_copies = card_copies[..card_index-1].to_vec();

    println!("{:?}", card_copies.iter().sum::<i32>() + card_index as i32);
}
