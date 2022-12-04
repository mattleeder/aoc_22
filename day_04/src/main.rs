use common::utils;

fn main() {
    let contents = utils::read_file().unwrap();

    let mut count = 0;

    for line in contents.lines() {
        let v: Vec<&str> = line.split(',').collect();
        let first: Vec<&str> = v[0].split('-').collect();
        let second: Vec<&str> = v[1].split('-').collect();
        let first_lower = first[0].parse::<u32>().unwrap();
        let first_upper = first[1].parse::<u32>().unwrap();
        let second_lower = second[0].parse::<u32>().unwrap();
        let second_upper = second[1].parse::<u32>().unwrap();

        if first_lower <= second_lower && first_upper >= second_upper {
            count += 1;
        } else if second_lower <= first_lower && second_upper >= first_upper {
            count += 1;
        }
    }

    println!("{}", count);
}
