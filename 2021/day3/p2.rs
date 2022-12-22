use std::fs;

fn main() {
    let input = fs::read_to_string("input1.txt").expect("Unable to read input");

    let mut fvec = input.lines().collect::<Vec<_>>();
    let usedbits = fvec[0].len();

    for i in 0..usedbits {
        let sumc: usize = fvec
            .iter()
            .fold(0, |acc, k| acc + (k.chars().nth(i) == Some('1')) as usize);
        let thechar = if sumc >= fvec.len() - sumc { '1' } else { '0' };
        fvec = fvec
            .into_iter()
            .filter(|k| k.chars().nth(i) == Some(thechar))
            .collect::<Vec<_>>();
        if fvec.len() == 1 {
            break;
        }
    }
    println!("{:?}", fvec);
    let oxy = fvec[0]
        .chars()
        .fold(0, |acc, k| (acc << 1) ^ k.to_digit(10).unwrap_or(0));

    fvec = input.lines().collect::<Vec<_>>();
    let usedbits = fvec[0].len();

    for i in 0..usedbits {
        let sumc: usize = fvec
            .iter()
            .fold(0, |acc, k| acc + (k.chars().nth(i) == Some('1')) as usize);
        let thechar = if sumc >= fvec.len() - sumc { '0' } else { '1' };
        fvec = fvec
            .into_iter()
            .filter(|k| k.chars().nth(i) == Some(thechar))
            .collect::<Vec<_>>();
        if fvec.len() == 1 {
            break;
        }
    }
    println!("{:?}", fvec);
    let carboxy = fvec[0]
        .chars()
        .fold(0, |acc, k| (acc << 1) ^ k.to_digit(10).unwrap_or(0));

    println!("{}", oxy * carboxy);
}
