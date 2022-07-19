use std::fs;

fn main() {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    let _input = fs::read_to_string("input1.txt")
        .expect("File is probably not here")
        .lines()
        .for_each(|elem| {
            //println!("{}", elem);
            let mut iter = elem.split_whitespace();
            let dir = iter.next().unwrap();
            let val = iter.next().unwrap().parse::<i32>().unwrap();
            match dir {
                "forward" => {
                    x += val;
                    y += aim * val;
                }
                "down" => aim += val,
                "up" => aim -= val,
                _ => panic!("Unknown direction"),
            }
            //println!("{} {}", dir, val);
        });
    println!("{}, {}, {}", x, y, x * y);
}
