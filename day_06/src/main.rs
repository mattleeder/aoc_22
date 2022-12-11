use common::utils;

fn main() {
    let mut char_map: [usize; 26] = [0; 26];
    let mut unique_count = 0;


    let binding = utils::read_file().unwrap();
    let mut binding = binding.chars();
    let contents: Vec<char> = (&mut binding).take(4).collect();
    let mut char_queue = Vec::new();

    for c in contents {
        if char_map[c as usize - 97] == 0 {
            unique_count += 1;
        }
        char_map[c as usize - 97] += 1;
        char_queue.insert(0, c);
    }

    let mut index = 4;

    for c in binding {
        if unique_count == 4 {
            println!("Result: {}", index);
            break;
        }

        let remove = char_queue.pop().unwrap();
        char_map[remove as usize - 97] -= 1;
        if char_map[remove as usize - 97] == 0 {
            unique_count -= 1;
        }
        
        if char_map[c as usize - 97] == 0 {
            unique_count += 1;
        }
        char_map[c as usize - 97] += 1;
        char_queue.insert(0, c);
        index += 1;
    }
}
