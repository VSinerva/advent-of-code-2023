use std::io;

fn main()
{
    let input = io::stdin();
    let mut input_str = String::new();

    let _ = input.read_line(&mut input_str);
    let time: u128 = input_str.split(":").last().unwrap().trim().replace(" ", "").parse::<u128>().unwrap();

    let _ = input.read_line(&mut input_str);
    let dist: u128 = input_str.split(":").last().unwrap().trim().replace(" ", "").parse::<u128>().unwrap();


    // Could also figure out the range with two binary searches, but that wasn't necessary
    let mut sum = 0;
    for t in 1..time {
        sum += if wins(t, time, dist) { 1 } else { 0 };
    }

    println!("{}", sum);

    let _ = input.read_line(&mut input_str);
}

fn wins(time: u128, max_time: u128, dist: u128) ->  bool {
    (max_time - time) * time > dist
}
