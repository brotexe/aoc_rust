use std::fs;

fn main() {
    let input = fs::read_to_string("../input1").unwrap();

    println!("Part 1: Delievered presents to {} houses", part_1(&input));
    println!("Part 2: Delievered presents to {} houses", part_2(&input));
}

fn move_entity (x: &mut i32, y: &mut i32, c: char) {
    match c {
        '^' => *y += 1,
        '>' => *x += 1,
        'v' => *y -= 1,
        '<' => *x -= 1,
        _ => {}        
    } 
}

fn part_1 (input: &str) -> i32 {
    let mut pos: Vec<(i32, i32)> = vec![];

    let (mut x, mut y) = (0, 0);
    let mut num_houses = 1;
    
    pos.push((x, y));

    for char in input.chars() {
        move_entity(&mut x, &mut y, char);
        if !pos.contains(&(x, y))  {
            pos.push((x, y));
            num_houses += 1;
        }
    }

    num_houses
}

fn part_2 (input: &str) -> i32 {
    let mut pos: Vec<(i32, i32)> = vec![];

    let (mut s_x, mut s_y) = (0, 0);
    let (mut r_x, mut r_y) = (0, 0);
    
    let mut num_houses = 1;
    
    pos.push((s_x, s_y));

    for (idx, char) in input.chars().enumerate() {
        if idx % 2 == 0 { 
            move_entity(&mut s_x, &mut s_y, char);
        } 
        else {
            move_entity(&mut r_x, &mut r_y, char);
        }

        if !pos.contains(&(s_x, s_y))  {
            pos.push((s_x, s_y));
            num_houses += 1;
        } 
        else if !pos.contains(&(r_x, r_y))  {
            pos.push((r_x, r_y));
            num_houses += 1;
        }
    }

    num_houses
}
