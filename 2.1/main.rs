use std::io;
use std::cmp::max;

fn main()
{
    let data = io::stdin().lines();

    let mut sum = 0;
    let total_r = 12;
    let total_g = 13;
    let total_b = 14;

    let mut game = 1;
    for line in data {
        let mut string = line.unwrap();
        if string.len() == 0 {
            break;
        }

        let mut max_r = 0;
        let mut max_g = 0;
        let mut max_b = 0;

        string = string.replace(" ", "");
        string = string.replace("red", "r");
        string = string.replace("green", "g");
        string = string.replace("blue", "b");
        string = string.replace(";", ",");
        string = string.split(":").last().unwrap().to_string();

        let colors = string.split(",");

        for color in colors {
            let mut color_string = color.to_string();
            let color_char = color_string.pop().unwrap();

            let val = color_string.parse::<i32>().unwrap();
            if color_char == 'r' {
                max_r = max(max_r, val);
            } else if color_char == 'g' {
                max_g = max(max_g, val);
            } else {
                max_b = max(max_b, val);
            }
        }

        if max_r <= total_r && max_g <= total_g && max_b <= total_b {
            sum += game;
        }

        game += 1;
    }

    println!("{}", sum);
}