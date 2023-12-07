use std::{io, collections::HashMap};

#[derive(Debug, PartialEq, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn new(hand: &str) -> HandType {
        let string = hand.to_string();

        let mut map: HashMap<char, i32> = HashMap::new();

        for c in string.chars() {
            if let Some(count) = map.get(&c) {
                map.insert(c, count + 1);
            } else {
                map.insert(c, 1);
            };
        }

        let mut count_vec: Vec<_> = map.into_values().collect();
        count_vec.sort();
        count_vec.reverse();

        let max = count_vec[0];
        let second = *count_vec.get(1).unwrap_or(&0);

        if max == 1 {
            HandType::HighCard
        } else if max == 2 && second != 2 {
            HandType::OnePair
        } else if max == 2 {
            HandType::TwoPair
        } else if max == 3 && second != 2 {
            HandType::ThreeOfAKind
        } else if max == 3 {
            HandType::FullHouse
        } else if max == 4 {
            HandType::FourOfAKind
        } else {
            HandType::FiveOfAKind
        }
    }

    fn hand_rank(&self) -> i32 {
        match self {
            HandType::FiveOfAKind => 0,
            HandType::FourOfAKind => 1,
            HandType::FullHouse => 2,
            HandType::ThreeOfAKind => 3,
            HandType::TwoPair => 4,
            HandType::OnePair => 5,
            HandType::HighCard => 6,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Play {
    pub hand_string: String,
    pub hand_type: HandType,
    pub bid: i32,
}

impl Play {
    fn new(hand_string: &str, bid: i32) -> Play {
        Play {
            bid,
            hand_type: HandType::new(hand_string),
            hand_string: hand_string.to_string(),
        }
    }
}

fn main()
{
    let char_ranks = "AKQJT98765432";
    let input = io::stdin().lines();

    let mut plays: Vec<Play> = Vec::new();
    for line in input {
        let string = line.unwrap();
        if string.len() == 0 {
            break;
        }

        let parts: Vec<_> = string.split_whitespace().collect();
        let hand = parts[0];
        let bid: i32 = parts[1].parse().unwrap();

        plays.push(Play::new(hand, bid));
    }

    plays.sort_by(|a, b| {
        if a.hand_type != b.hand_type {
            b.hand_type.hand_rank().cmp(&a.hand_type.hand_rank())
        } else {
            for i in 0..a.hand_string.len() {
                let ac: char = a.hand_string.chars().nth(i).unwrap();
                let bc: char = b.hand_string.chars().nth(i).unwrap();
                if ac != bc {
                    return  char_ranks.find(bc).unwrap().cmp(&char_ranks.find(ac).unwrap());
                }
            }
            b.bid.cmp(&a.bid)
        }
    });

    let mut total = 0;
    for (i, play) in plays.iter().enumerate() {
        total += (i+1) * play.bid as usize;
    }

    println!("{}", total);
}
