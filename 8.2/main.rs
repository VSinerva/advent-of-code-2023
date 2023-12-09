use std::{io, collections::{HashMap, HashSet}};

fn main()
{
    let input = io::stdin();
    let mut instruction_string = String::new();
    let _ = input.read_line(&mut instruction_string);
    instruction_string = instruction_string.trim().to_string();
    let _ = input.read_line(&mut String::new());

    let mut starts: Vec<_> = Vec::new();
    let mut ends: Vec<_> = Vec::new();
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

        if start.chars().nth(2).unwrap() == 'A' {
            starts.push((0_usize, start.clone()));
        }
        if start.chars().nth(2).unwrap() == 'Z' {
            ends.push(start.clone());
        }

        l.insert(start.clone(), l_dest);
        r.insert(start, r_dest);
    }

    let mut next_end: HashMap<(String, usize), (String, usize)> = HashMap::new();

    for start in ends {
        for i in 0..instruction_string.len() {
            let mut prev: HashSet<(String, usize)> = HashSet::new();
            let mut steps = 0_usize;
            let mut pos = start.clone();
            let mut char_i = i;
            while steps < 1 ||  pos.chars().nth(2).unwrap() != 'Z' {
                let c = instruction_string.chars().nth(char_i).unwrap();
                if c == 'L' {
                    pos = l[&pos].clone();
                } else if c == 'R' {
                    pos = r[&pos].clone();
                }

                if prev.contains(&(pos.clone(), char_i)) {
                    break;
                }
                prev.insert((pos.clone(), char_i));

                steps += 1;
                char_i = (char_i + 1) % instruction_string.len();
            }
            if pos.chars().nth(2).unwrap() != 'Z' {
                break;
            }

            next_end.insert((start.clone(), i), (pos, steps));
        }
    }

    let mut positions: Vec<_> = Vec::new();
    for (mut steps, mut pos) in starts{
        while pos.chars().nth(2).unwrap() != 'Z' {
            for c in instruction_string.chars() {
                if c == 'L' {
                    pos = l[&pos.clone()].clone();
                } else if c == 'R' {
                    pos = r[&pos.clone()].clone();
                }
                steps += 1;
            }
        }
        positions.push((steps, pos));
    }

    let mut done = false;

    while !done {
        done = true;
        positions.sort();

        for i in 0..positions.len()-1 {
            if positions[i].0 != positions[i+1].0 {
                let mut steps = positions[i].0;
                let mut pos = positions[i].1.clone();
                let tuple = (pos.clone(), steps % instruction_string.len());
                steps += next_end[&tuple].1;
                pos = next_end[&tuple].0.clone();

                positions[i] = (steps, pos);

                done = false;
                break;
            }
        }
    }

    println!("{:?}", positions[0].0);
}
