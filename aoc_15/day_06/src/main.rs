use std::fs;

fn main() {
    let input = fs::read_to_string("../input").unwrap();

    println!("Part 1: there are {} lights lit", part_1(&input));
    println!("Part 2: there is {} total bringhtness", part_2(&input));
}

fn part_1(input: &str) -> u32 {
    let mut lights: [[u32; 1000]; 1000] = [[0; 1000]; 1000];
    for line in input.split("\n") {
       simulate_event(line, &mut lights, 1); 
    }
    
    let mut lit_lights = 0;

    for x in 0..1000 {
        for y in 0..1000 {
            if lights[x][y] == 1 {
                lit_lights += 1;
            }
        }
    }

    lit_lights
}

fn part_2(input: &str) -> u32 {
    let mut lights: [[u32; 1000]; 1000] = [[0; 1000]; 1000];
    for line in input.split("\n") {
       simulate_event(line, &mut lights, 2); 
    }
    
    let mut total_brightness = 0;

    for x in 0..1000 {
        for y in 0..1000 {
            total_brightness += lights[x][y];            
        }
    }

    total_brightness
}

fn simulate_event(event: &str, mut lights: &mut [[u32; 1000]; 1000], part: u32) {
    let events:Vec<&str> = event.split(" ").collect();
    match events[0] {
        "toggle" => toggle(events[1], events[3], &mut lights, part),
        "turn" => {
            if events[1] == "on" {
                turn_on(events[2], events[4], &mut lights, part);
            } else { turn_off(events[2], events[4], &mut lights, part); }
        },
        _ => {}
    }
}

fn turn_on(initial_coordinates: &str, final_coordinates: &str, lights: &mut [[u32; 1000]; 1000], part: u32) {
    let (initial_x, initial_y) = parse_coordinates(initial_coordinates);
    let (final_x, final_y) = parse_coordinates(final_coordinates);

    for x in initial_x..=final_x {
        for y in initial_y..=final_y {
            if part == 1 {
                lights[x][y] = 1;
            } else {
                lights[x][y] += 1;
            }
        }
    }

}

fn turn_off(initial_coordinates: &str, final_coordinates: &str, lights: &mut [[u32; 1000]; 1000], part: u32) {
    let (initial_x, initial_y) = parse_coordinates(initial_coordinates);
    let (final_x, final_y) = parse_coordinates(final_coordinates);

    for x in initial_x..=final_x {
        for y in initial_y..=final_y {
            if part == 1 {
                lights[x][y] = 0;
            } else {
                if lights[x][y] == 0 {
                    lights[x][y] = 0;
                } else {
                    lights[x][y] -= 1;
                }
            }
        }
    }
}

fn toggle(initial_coordinates: &str, final_coordinates: &str, lights: &mut [[u32; 1000]; 1000], part: u32) {
    let (initial_x, initial_y) = parse_coordinates(initial_coordinates);
    let (final_x, final_y) = parse_coordinates(final_coordinates);

   for x in initial_x..=final_x {
        for y in initial_y..=final_y {
            if part == 1 {
                if lights[x][y] == 1 {
                    lights[x][y] = 0;
                } else { lights[x][y] = 1 }
            } else {
                lights[x][y] += 2;
            }
        }
    }
}

fn parse_coordinates(coordinates: &str) -> (usize, usize) {
    let (x, y) = (
            coordinates.split(",").nth(0).unwrap()
                .parse::<usize>().unwrap(),
            coordinates.split(",").nth(1).unwrap()
                .parse::<usize>().unwrap(),
    );
    (x, y)
}

