use common::utils;
use std::cmp::Ordering;

#[derive(Debug)]
enum VecEntry {
    SubVec(Vec<VecEntry>),
    Num(u32),
}

impl VecEntry {
    
    fn push(&mut self, item: VecEntry) {
        match self {
            VecEntry::SubVec(vec) => {vec.push(item)},
            VecEntry::Num(_) => panic!("Cannot call push on VecEntry::Char"),
        }
    }

}

impl Ord for VecEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for VecEntry {}

impl PartialOrd for VecEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // If both SubVec, ind == 10,
        // If both Num, ind == 5
        // If self SubVec and other Num, ind == 9
        // If self Num and other SubVec, ind == 6
        let mut ind = 0_u8;
        ind += if matches!(self, VecEntry::SubVec(_)) {8} else {4};
        ind += if matches!(other, VecEntry::SubVec(_)) {2} else {1};

        match ind {
            10 => {
                let s = if let VecEntry::SubVec(s) = self { s } else { todo!() };
                let o = if let VecEntry::SubVec(o) = other { o } else { todo!() };
                if s.lt(o) {
                    return Some(Ordering::Less)
                } else if s.gt(o) {
                    return Some(Ordering::Greater)
                } else {
                    return Some(Ordering::Equal)
                }
            },
            9 => {
                let s = if let VecEntry::SubVec(s) = self { s } else { todo!() };
                let o = if let VecEntry::Num(o) = other { o } else { todo!() };
                if s.len() == 0 {
                    return Some(Ordering::Less)
                } else if s[0].gt(&VecEntry::Num(o.to_owned())) {
                    return Some(Ordering::Greater)
                } else if s[0].lt(&VecEntry::Num(o.to_owned())) {
                    return Some(Ordering::Less)
                } else {
                    // If self has more items then self is greater than other
                    if s.len() == 1 {
                        return Some(Ordering::Equal)
                    } else {
                        return Some(Ordering::Greater)
                    }
                }
            },
            6 => {
                let s = if let VecEntry::Num(s) = self { s } else { todo!() };
                let o = if let VecEntry::SubVec(o) = other { o } else { todo!() };
                if o.len() == 0 {
                    return Some(Ordering::Greater)
                } else if o[0].lt(&VecEntry::Num(s.to_owned())) {
                    return Some(Ordering::Greater)
                } else if o[0].gt(&VecEntry::Num(s.to_owned())) {
                    return Some(Ordering::Less)
                } else {
                    // If other has more items then self is less than other
                    if o.len() == 1 {
                        return Some(Ordering::Equal)
                    } else {
                        return Some(Ordering::Less)
                    }
                }
            },
            5 => {
                let s = if let VecEntry::Num(s) = self { s } else { todo!() };
                let o = if let VecEntry::Num(o) = other { o } else { todo!() };
                if s < o {
                    return Some(Ordering::Less)
                } else if s > o {
                    return Some(Ordering::Greater)
                } else {
                    return Some(Ordering::Equal)
                }
            },
            _ => {panic!("Ind total unexpected in VecEntry PartialOrd")},
        }
    }
}

impl PartialEq for VecEntry {
    fn eq(&self, other: &Self) -> bool {
        // If both SubVec, ind == 10,
        // If both Num, ind == 5
        // If self SubVec and other Num, ind == 9
        // If self Num and other SubVec, ind == 6
        let mut ind = 0_u8;
        ind += if matches!(self, VecEntry::SubVec(_)) {8} else {4};
        ind += if matches!(other, VecEntry::SubVec(_)) {2} else {1};

        match ind {
            10 => {
                let s = if let VecEntry::SubVec(s) = self { s } else { todo!() };
                let o = if let VecEntry::SubVec(o) = other { o } else { todo!() };
                return s.eq(o)
            },
            9 => {
                let s = if let VecEntry::SubVec(s) = self { s } else { todo!() };
                let o = if let VecEntry::Num(o) = other { o } else { todo!() };
                if s.len() != 1 {
                    return false
                }
                return s[0].eq(&VecEntry::Num(o.to_owned()))
            },
            6 => {
                let s = if let VecEntry::Num(s) = self { s } else { todo!() };
                let o = if let VecEntry::SubVec(o) = other { o } else { todo!() };
                if o.len() != 1 {
                    return false
                }
                return o[0].eq(&VecEntry::Num(s.to_owned()))

            },
            5 => {
                let s = if let VecEntry::Num(s) = self { s } else { todo!() };
                let o = if let VecEntry::Num(o) = other { o } else { todo!() };
                return s.eq(o)
            },
            _ => {panic!("Ind total unexpected in VecEntry PartialEq")},
        }
    }
}

fn read_vec(line: &Vec<char>, mut index: usize) -> (usize, VecEntry) {
    let length: usize = line.len();
    index += 1;
    let mut v: VecEntry = VecEntry::SubVec(Vec::new());
    let mut curr: Vec<char> = Vec::new();
    while index < length {
        if line[index] == '[' {
            let (new_index, sub_vec) = read_vec(line, index);
            v.push(sub_vec);
            index = new_index;
        } else if line[index] == ']' {
            // println!("Curr: {:?}", curr);
            if curr.len() > 0 {
                let n = curr.iter().collect::<String>();
                let n = n.parse::<u32>().unwrap();
                v.push(VecEntry::Num(n));
            }
            return (index, v)
        } else if line[index] == ',' {
            // v.push(VecEntry::Char(line[index]));
            if curr.len() > 0 {
                let n = curr.iter().collect::<String>();
                let n = n.parse::<u32>().unwrap();
                v.push(VecEntry::Num(n));
                curr.clear();
            }
        } else {
            curr.push(line[index]);
        }
        index += 1
    }

    if curr.len() > 0 {
        let n = curr.iter().collect::<String>();
        let n = n.parse::<u32>().unwrap();
        v.push(VecEntry::Num(n));
        curr.clear();
    }

    (index, v)

}

fn read_line(line: &str) -> Vec<VecEntry> {
    let line: Vec<char> = line.chars().collect();
    let mut index: usize = 0;
    let length = line.len();
    let mut v: Vec<VecEntry> = Vec::new();

    while index < length {
        if line[index] == '[' {
            let (new_index, sub_vec) = read_vec(&line, index);
            index = new_index;
            v.push(sub_vec);
        }
        index += 1;
    }

    v
}

fn main() {
    let binding = utils::read_file().unwrap();
    let contents = binding.lines();

    let mut v: Vec<VecEntry> = contents.filter_map(|line| {
        if line == "" {
            None
        } else {
            let mut buffer = read_line(line);
            Some(buffer.pop().unwrap())
        }
    }).collect();
    v.push(VecEntry::SubVec(vec![VecEntry::Num(2)]));
    v.push(VecEntry::SubVec(vec![VecEntry::Num(6)]));
    v.sort();

    let key_one = v.binary_search(&VecEntry::SubVec(vec![VecEntry::Num(2)])).unwrap() + 1;
    let key_two = v.binary_search(&VecEntry::SubVec(vec![VecEntry::Num(6)])).unwrap() + 1;


    println!("Part 2: {}", key_one * key_two);
}
