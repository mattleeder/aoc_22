use common::utils;
use std::collections::HashMap;

fn main() {
    let keys = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut char_map: HashMap<char, usize> = HashMap::new();
    let mut appeared: [usize; 58] = [0; 58];

    for (i, c) in keys.chars().enumerate() {
        char_map.insert(c, i + 1);
    }

    let contents = utils::read_file().unwrap();

    let mut total_priorty = 0;

    for line in contents.lines() {
        let mut seen = appeared.clone();
        let half = line.len() / 2;
        for letter in line[..half].chars() {
            seen[letter as usize - 65] = 1;
        }
        for letter in line[half..].chars() {
            if seen[letter as usize - 65] == 1 {
                total_priorty += char_map[&letter];
                break;
            }
        }
    }

    println!("{}", total_priorty);
}
