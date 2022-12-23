use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|line| line.to_owned())
        .collect::<Vec<_>>();
    let mut val_ar: Vec<u8> = vec![0; 53];
    let mut ans: i32 = 0;
    for ctr in 0..=input.len() {
        if ctr % 3 == 0 {
            if ctr == 0 {
            } else {
                let increment = val_ar
                    .iter()
                    .enumerate()
                    .max_by_key(|(_idx, &val)| val)
                    .map(|(idx, _val)| idx)
                    .unwrap() as i32;
                ans += increment;
                // println!("{} {}", increment, ctr);
                // println!("{:?}", val_ar);
                val_ar.fill(0);
            }
            if ctr == input.len() {
                break;
            }
        }
        input[ctr].chars().for_each(|c| {
            let i = if c.is_uppercase() {
                c as usize - 65 + 27
            } else {
                c as usize - 96
            };
            if i >= 53 {
                // println!("{} {}", c, i);
            }
            val_ar[i] += if val_ar[i] == (ctr % 3) as u8 { 1 } else { 0 };
        });
    }
    println!("{}", ans);
}
