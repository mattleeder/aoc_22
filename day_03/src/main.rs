use common::utils;
use std::collections::HashMap;

fn main() {
    let keys = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut char_map: HashMap<char, usize> = HashMap::new();
    let appeared: [usize; 58] = [0; 58];

    for (i, c) in keys.chars().enumerate() {
        char_map.insert(c, i + 1);
    }

    let binding = utils::read_file().unwrap();
    let mut contents = binding.lines();

    let mut total_priorty = 0;

    while let (Some(line1), Some(line2), Some(line3)) =
        (contents.next(), contents.next(), contents.next())
    {
        let mut seen = appeared.clone();
        println!("NEW CHUNK!");
        println!("{}\n{}\n{}", line1, line2, line3);
        for letter in line1.chars() {
            seen[letter as usize - 65] = 1;
        }
        for letter in line2.chars() {
            if seen[letter as usize - 65] == 1 {
                seen[letter as usize - 65] = 2;
            }
        }
        for letter in line3.chars() {
            if seen[letter as usize - 65] == 2 {
                total_priorty += char_map[&letter];
                println!("{}\n", letter);
                break;
            }
        }
    }

    println!("{}", total_priorty);
}
