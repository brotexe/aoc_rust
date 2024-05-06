fn main() {
    let input = "ckczppom";

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> i32 {
    let mut i = 1;
    loop {
        let hash = format!("{:x}", md5::compute(input.to_owned() + &i.to_string()));
        if &hash[..5] == "00000" {
            return i;
        }
        i += 1 ;
    }
}

fn part_2(input: &str) -> i32 {
    let mut i = 1;
    loop {
        let hash = format!("{:x}", md5::compute(input.to_owned() + &i.to_string()));
        if &hash[..6] == "000000" {
            return i;
        }
        i += 1 ;
    }
}
