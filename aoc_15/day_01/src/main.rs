use std::fs;

fn main() {
    let input = fs::read_to_string("../input").unwrap();
    let mut current_floor = 0;
    let mut neg_floor_idx = 0;

    for (idx, char) in input.chars().enumerate() {
        match char {
            '(' => current_floor += 1,
            ')' => current_floor -= 1,
            _ => {}        
        }
        if current_floor == -1 && neg_floor_idx == 0 {
            neg_floor_idx = idx;
        }
    }

    println!("Part 1: The current floor is {current_floor}");
    println!("Part 2: The index is {neg_floor_idx}");
}
