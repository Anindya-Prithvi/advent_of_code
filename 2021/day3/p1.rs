use std::fs;

fn main() {
    let mut inputbits = vec![0]; // counts the on bits
    let mut len = 0;
    let mut usedbits = 0;
    let _input = fs::read_to_string("input1.txt")
        .expect("Unable to read input")
        .trim()
        .lines()
        .map(|line| {
            let linelength = line.len();
            if inputbits.len() != linelength {
                inputbits.resize(linelength, 0);
                usedbits = linelength;
            }
            len += 1;
            for i in 0..linelength {
                match line.chars().nth(i) {
                    Some('1') => inputbits[i] += 1,
                    Some('0') => (),
                    _ => panic!("Invalid input"),
                }
            }
            0
        })
        .collect::<Vec<i32>>();
    println!("{:?}", inputbits);

    let mut epsilon = vec![false; usedbits];
    let mut delta = vec![false; usedbits];

    for i in 0..usedbits {
        epsilon[i] = inputbits[i] > len / 2;
        delta[i] = !epsilon[i];
    }

    let dep = epsilon.iter().fold(0, |acc, x| (acc << 1) ^ *x as i32);
    let dde = delta.iter().fold(0, |acc, x| (acc << 1) ^ *x as i32);
    println!("{}", dep * dde);
}
