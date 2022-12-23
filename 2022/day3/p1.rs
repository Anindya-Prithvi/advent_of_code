use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|x| {
            let mut ocs: Vec<bool> = vec![false; 53]; // 1 based
            let mut result: u8 = 0;
            let mut num_it = 0;
            for c in x.chars() {
                let i = if c.is_uppercase() {
                    c as u8 - 65 + 27
                } else {
                    c as u8 - 96
                };
                if ocs[i as usize] == true {
                    result = i;
                } else if num_it < x.len() / 2 {
                    ocs[i as usize] = true;
                }
                num_it += 1;
            }
            result as u64
        })
        .into_iter()
        .sum::<u64>();
    println!("{:?}", input);
}
