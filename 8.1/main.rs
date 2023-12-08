use std::{io, collections::HashMap};

fn main()
{
    let input = io::stdin();
    let mut instruction_string = String::new();
    let _ = input.read_line(&mut instruction_string);
    instruction_string = instruction_string.trim().to_string();
    let _ = input.read_line(&mut String::new());

    let mut l: HashMap<String, String> = HashMap::new();
    let mut r: HashMap<String, String> = HashMap::new();

    for line in input.lines() {
        let string = line.unwrap();
        if string.len() == 0 {
            break;
        }

        let start = string[0..3].to_string();
        let l_dest = string[7..10].to_string();
        let r_dest = string[12..15].to_string();
        l.insert(start.clone(), l_dest);
        r.insert(start, r_dest);
    }

    let mut total = 0;
    let mut pos = String::from("AAA");

    while pos != String::from("ZZZ") {
        for c in instruction_string.chars() {
            if c == 'L' {
                pos = l[&pos].clone();
            } else if c == 'R' {
                pos = r[&pos].clone();
            }

            total += 1;

            if *pos == String::from("ZZZ") {
                break;
            }
        }
    }

    println!("{:?}", total);
}
