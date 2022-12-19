use common::utils;
use std::fmt;

struct Monkey {
    id: u8,
    items: Vec<u64>,                    // Items carried
    operation: Box<dyn Fn(u64) -> u64>, // How worry is manuipulated
    test: u64,                          // The boolean test divisor
    if_true: u8,                        // Id of Monkey to throw to if true
    if_false: u8,                       // Id of Monkey to throw to if false
    throw_count: u64,
    threshold: u64,
}

impl Monkey {
    fn reverse_items(&mut self) {
        self.items.reverse();
    }

    fn inspect_item(&mut self, item: u64) -> (u8, u64) {
        let mut item = (self.operation)(item);
        item = item % self.threshold;
        self.throw_count += 1;
        if item % self.test == 0 {
            return (self.if_true, item);
        } else {
            return (self.if_false, item);
        }
    }
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Monkey {}:
  Starting items: {:?}
  Operation: {:?}
  Test: {}
    If true: {}
    If false: {}",
            self.id, self.items, "Blank", self.test, self.if_true, self.if_false
        )
    }
}

struct MonkeyReader {
    id: Option<u8>,
    items: Option<Vec<u64>>,
    operation: Option<Box<dyn Fn(u64) -> u64>>,
    test: Option<u64>,
    if_true: Option<u8>,
    if_false: Option<u8>,
    monkeys: Vec<Monkey>,
    test_vec: Vec<u64>,
}

impl MonkeyReader {
    fn new() -> MonkeyReader {
        MonkeyReader {
            id: None,
            items: None,
            operation: None,
            test: None,
            if_true: None,
            if_false: None,
            monkeys: Vec::new(),
            test_vec: Vec::new(),
        }
    }

    fn create_monkey(&mut self) -> Monkey {
        Monkey {
            id: self.id.take().unwrap(),
            items: self.items.take().unwrap(),
            operation: self.operation.take().unwrap(),
            test: self.test.take().unwrap(),
            if_true: self.if_true.take().unwrap(),
            if_false: self.if_false.take().unwrap(),
            throw_count: 0,
            threshold: 0,
        }
    }

    fn clear(&mut self) {
        self.id = None;
        self.items = None;
        self.operation = None;
        self.test = None;
        self.if_true = None;
        self.if_false = None;
    }

    fn get_monkeys(&mut self) -> Vec<Monkey> {
        self.monkeys.drain(0..).collect()
    }

    fn read_line(&mut self, line: &str) {
        let line: Vec<&str> = line.split(":").collect();
        let instruction = line[0].trim();
        if instruction.starts_with("Monkey") {
            // Start new Monkey
            self.clear();
            self.id = Some(MonkeyReader::read_id(instruction));
        } else if instruction.starts_with("Starting items") {
            self.items = Some(MonkeyReader::read_items(line[1].trim()));
        } else if instruction.starts_with("Operation") {
            self.operation = Some(MonkeyReader::read_operation(line[1].trim()));
        } else if instruction.starts_with("Test") {
            let test = MonkeyReader::read_test(line[1].trim());
            self.test_vec.push(test);
            self.test = Some(test.to_owned());
        } else if instruction.starts_with("If true") {
            self.if_true = Some(MonkeyReader::read_true_target(line[1].trim()));
        } else if instruction.starts_with("If false") {
            self.if_false = Some(MonkeyReader::read_false_target(line[1].trim()));
        } else {
            // Push monkey
            let monkey = self.create_monkey();
            self.monkeys.push(monkey);
        }
    }

    fn read_id(s: &str) -> u8 {
        let n = s.split(" ").nth(1).unwrap();
        n.parse::<u8>().unwrap()
    }

    fn read_items(s: &str) -> Vec<u64> {
        s.trim()
            .split(", ")
            .map(|item| item.parse::<u64>().unwrap())
            .collect()
    }

    fn read_operation(s: &str) -> Box<dyn Fn(u64) -> u64> {
        let rhs = s.split("=").nth(1).unwrap().trim();
        let v: Vec<&str> = rhs.split(" ").collect();
        let left_arg = if v[0] == "old" {
            0_u64
        } else {
            v[0].parse::<u64>().unwrap()
        };
        let right_arg = if v[2] == "old" {
            0_u64
        } else {
            v[2].parse::<u64>().unwrap()
        };
        let op = match v[1] {
            "*" => MonkeyReader::mult,
            "+" => MonkeyReader::add,
            "-" => MonkeyReader::sub,
            "/" => MonkeyReader::div,
            _ => {
                panic!("Unexptected operation during read_operation")
            }
        };

        if v[0] == "old" && v[2] == "old" {
            return Box::new(move |x| op(x, x));
        } else if v[0] == "old" {
            return Box::new(move |x| op(x, right_arg));
        } else if v[2] == "old" {
            return Box::new(move |x| op(x, left_arg));
        } else {
            return Box::new(move |_x| op(left_arg, right_arg));
        }
    }

    fn add(x: u64, y: u64) -> u64 {
        x + y
    }

    fn mult(x: u64, y: u64) -> u64 {
        x * y
    }

    fn div(x: u64, y: u64) -> u64 {
        x / y
    }

    fn sub(x: u64, y: u64) -> u64 {
        x - y
    }

    fn read_test(s: &str) -> u64 {
        s.split(" ").last().unwrap().parse::<u64>().unwrap()
    }

    fn read_true_target(s: &str) -> u8 {
        s.split(" ").last().unwrap().parse::<u8>().unwrap()
    }

    fn read_false_target(s: &str) -> u8 {
        s.split(" ").last().unwrap().parse::<u8>().unwrap()
    }
}

fn main() {
    let mut reader = MonkeyReader::new();
    let contents = utils::read_file().unwrap();
    for line in contents.lines() {
        reader.read_line(line);
    }
    let monkey = reader.create_monkey();
    reader.monkeys.push(monkey);

    println!("Tests: {:?}", reader.test_vec);
    let threshold = reader.test_vec.iter().product::<u64>();
    println!("Tests product: {}", threshold);
    let mut monkey_vec: Vec<Monkey> = reader.get_monkeys();
    println!("MonkeyVec len: {}", monkey_vec.len());

    for i in 0..monkey_vec.len() {
        monkey_vec[i].threshold = threshold;
    }

    for _ in 0..10_000 {
        for i in 0..monkey_vec.len() {
            let mut v: Vec<(u8, u64)> = Vec::new();
            monkey_vec[i].reverse_items();
            for j in 0..monkey_vec[i].items.len() {
                let item = monkey_vec[i].items[j].to_owned();
                v.push(monkey_vec[i].inspect_item(item));
            }
            monkey_vec[i].items.clear();
            for (monkey, item) in v.iter() {
                monkey_vec[monkey.to_owned() as usize]
                    .items
                    .push(item.to_owned());
            }
        }
    }

    let mut throws: Vec<u64> = Vec::new();
    for i in 0..monkey_vec.len() {
        println!("Pushing Monkey {}: {}", i, monkey_vec[i].throw_count);
        println!("Monkey holding items: {:?}", monkey_vec[i].items);
        throws.push(monkey_vec[i].throw_count);
    }

    throws.sort();

    println!("{:?}", throws);
    println!("Part 2: {}", throws.last().unwrap() * throws[throws.len() - 2]);
}
