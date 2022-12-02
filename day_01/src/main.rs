use common::utils;

fn main() {

    let contents = utils::read_file().unwrap();
    let mut max = [0_i64; 3];
    let mut cur: i64 = 0;

    for line in contents.lines() {
        match line {
            "" => {
                if cur > max[0] {
                    max[0] = cur;
                }
                cur = 0;
                max.sort();
            },
            _ => cur += line.parse::<i64>().unwrap(),
        }
    }

    if cur > max[0] {
        max[0] = cur;
    }

    let mut res: i64 = 0;

    for num in max.iter() {
        res += num;
    }
    
    println!("{}", res);
}
