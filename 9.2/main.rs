use std::io;

fn main()
{
    let input = io::stdin().lines();

    let mut sum = 0;

    for line in input {
        let string = line.unwrap();
        if string.len() == 0 {
            break;
        }

        let mut vec: Vec<i32> = string.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        extrapolate(&mut vec);
        sum += vec[0];
    }

    println!("{:?}", sum);
}

fn extrapolate (vec: &mut Vec<i32>) {
    let mut diff_vec: Vec<_> = vec![0; vec.len() - 1];

    let mut zeroes = true;
    for i in 0..diff_vec.len() {
        diff_vec[i] = vec[i+1] - vec[i];
        if diff_vec[i] != 0 {
            zeroes = false;
        }
    }

    if zeroes {
        vec.push(vec[0]);
    } else {
        extrapolate(&mut diff_vec);
        vec.insert(0, vec[0] - diff_vec[0]);
    }
}
