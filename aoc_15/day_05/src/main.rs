use std::fs;

fn main() {
    let input = fs::read_to_string("../input").unwrap();
    //let input = "xxxyx";
    println!("Part 1: has {} nice words", part_1(&input));
    println!("Part 2: has {} nice words", part_2(&input));
}

fn part_1(input: &str) -> u32 {
    let mut num_nice_words = 0;
    for word in input.split("\n") {
        if has_three_vowels(word) && has_double_letter(word) && !has_prohibited_word(word) {
            num_nice_words += 1;
        }
    }

    num_nice_words
}

fn has_three_vowels(word: &str) -> bool {
    let mut num_vowels = 0;
    for c in word.chars() {
        match c { 
            'a' | 'e' | 'i' | 'o' | 'u' => num_vowels += 1,
            _ => { }
        }
        if num_vowels == 3 {
            return true;
        }
    }    
    false
}

fn has_double_letter(word: &str) -> bool {
    for i in 1..word.len() {
        if word.chars().nth(i - 1) == word.chars().nth(i) {
            return true;
        }
    }
    false
}

fn has_prohibited_word(word: &str) -> bool {
    for i in 0..(word.len() - 1) {
        match &word[i..(i + 2)] { 
            "ab" | "cd" | "pq" | "xy" => return true,
            _ => { }
        }
    }
    
    false
}

fn part_2(input: &str) -> u32 {
    let mut num_nice_words = 0;
    for word in input.trim().split("\n") {
        //println!("{}", word);
        if has_double_letter_separated(word) && has_separated_pair(word) {
            num_nice_words += 1;
        }
    }    

    num_nice_words
}

fn has_double_letter_separated(word: &str) -> bool {
    for i in 1..(word.len() - 1) {
        if word.chars().nth(i - 1) == word.chars().nth(i + 1) || 
           word.chars().nth(i) == word.chars().nth(i + 2) {
            return true;
        }
    }
    false
}

fn has_separated_pair(word: &str) -> bool {
    for i in 0..(word.len() - 1) {
        let substring = &word[i..=(i + 1)];
        
        for j in 0..(word.len() - 1) {
            if i == j || i == j + 1 || i + 1 == j {
                continue;
            }
            if &word[j..=(j + 1)] == substring {
                return true;
            }
        }
    }
    false
}
