use std::io;

fn main()
{
    let mut input = io::stdin().lines();

    let mut seeds_vec = Vec::new();
    for line in &mut input {
        let string = line.unwrap();
        if string.len() == 0 {
            break;
        }
        seeds_vec = string.split(":").last().unwrap().split_whitespace().map(|s| s.parse::<u128>().unwrap()).collect();
    }

    for _ in 0..7 {
        let _ = input.next();

        let mut new_seeds_vec = seeds_vec.clone();

        for line in &mut input {
            let string = line.unwrap();
            if string.len() == 0 {
                break;
            }

            let values: Vec<u128> = string.split_whitespace().map(|s| s.parse::<u128>().unwrap()).collect();
            let dest_start = values[0];
            let source_start = values[1];
            let range = values[2];

            for (i, seed) in seeds_vec.iter().enumerate() {
                if *seed >= source_start && *seed < source_start + range {
                    new_seeds_vec[i] = dest_start + (seed - source_start);
                }
            }
        }

        seeds_vec = new_seeds_vec;
    }

    println!("{:?}", seeds_vec.iter().min().unwrap());
}
