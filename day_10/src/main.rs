use common::utils;
use std::fs::File;
use std::io::Write;

struct CPU {
    cycles: u32,
    x_register: i32,
    log: File,
}

impl CPU {

    fn new(log: File) -> CPU {
        CPU { cycles: 0, x_register: 1, log }
    }

    fn increment_cycles(&mut self) {
        self.draw();
        self.cycles += 1;
    }

    fn noop(&mut self) {
        self.increment_cycles();
    }

    fn addx(&mut self, x: i32) {
        self.increment_cycles();
        self.increment_cycles();
        self.x_register += x;
    }

    fn draw(&mut self) {
        let log_line = format!("During Cycle: {}\t\tX Register: {}\n", self.cycles + 1, self.x_register);
        self.log.write_all(log_line.as_bytes()).unwrap();
        if (self.cycles) % 40 == 0 {
            print!("    Cycle: {}\n", self.cycles);
        }
        if self.is_sprite_aligned() {
            print!("#");
        } else {
            print!(".");
        }
    }

    fn is_sprite_aligned(&self) -> bool {
        self.x_register.abs_diff((self.cycles % 40) as i32) <= 1
    }

    fn read_instruction(&mut self, line: &str) {
        let line: Vec<&str> = line.split(" ").collect();
        match line[0] {
            "noop" => { self.noop() },
            "addx" => { self.addx(line[1].parse::<i32>().unwrap())},
            _ => { panic!("Unknown command") },
        }
    }

}

fn main() {
    let mut file = File::create("aoc_day_10_log.txt").unwrap();
    file.write_all(b"NEW LOG\n=====================================================\n").unwrap();
    let contents = utils::read_file().unwrap();
    let mut cpu = CPU::new(file);

    for line in contents.lines() {
        cpu.read_instruction(line);
    }
}
