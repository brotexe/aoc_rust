use std::fs;

fn main() {
    let input = fs::read_to_string("../input").unwrap();
    let mut wrapping_paper = 0;
    let mut ribbon = 0;

    for line in input.trim().split("\n") {
        let (l, w, h) = get_values(line);
        wrapping_paper += (l*w + w*h + h*l) * 2 + find_min(l*w, w*h, h*l);
        ribbon += find_smallest_perimeter(l, w, h) + l*w*h;    
    }

    println!("Part 1: total wrapping paper: {wrapping_paper}");
    println!("Part 2: total ribbon: {ribbon}");
}

fn get_values(line: &str) -> (i32, i32, i32) {
    let values: Vec<&str> = line.split("x").collect();
    (values[0].parse().unwrap(), values[1].parse().unwrap(), values[2].parse().unwrap())
}

fn find_min(l: i32, w: i32, h: i32) -> i32 {
    let mut min = l;
    if min > w {
        min = w;
    }

    if min > h {
        min = h;
    }

    min
}

fn find_smallest_perimeter(l: i32, w: i32, h: i32) -> i32 {
    find_min((l+w) * 2, (w+h) * 2, (h+l) * 2)
}
