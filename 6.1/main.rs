use std::io;

fn main()
{
    let input = io::stdin();
    let mut input_str = String::new();

    let _ = input.read_line(&mut input_str);
    let times_vec: Vec<i32> = input_str.split(":").last().unwrap().split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();

    let _ = input.read_line(&mut input_str);
    let dists_vec: Vec<i32> = input_str.split(":").last().unwrap().split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();

    let mut product = 1;

    for (i, time) in times_vec.into_iter().enumerate() {
        let mut sum = 0;
        for t in 1..time {
            sum += if wins(t, time, dists_vec[i]) { 1 } else { 0 };
        }

        product *= sum;
    }

    println!("{}", product);

    let _ = input.read_line(&mut input_str);
}

fn wins(time: i32, max_time: i32, dist: i32) ->  bool {
    (max_time - time) * time > dist
}
