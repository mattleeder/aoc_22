use common::utils;

struct CycleListener {
    start: u32,
    step: u32,
    total: i32,
}

impl CycleListener {

    fn new(start: u32, step: u32) -> CycleListener {
        CycleListener{ start, step, total: 0 }
    }

    fn is_speak_cycle(&self, cycles: u32) -> bool {
        if cycles >= self.start {
            return ((cycles - self.start) % self.step) == 0
        }
        false
    }

}

struct CPU {
    cycles: u32,
    x_register: i32,
    listener: CycleListener,
}

impl CPU {

    fn new(listener: CycleListener) -> CPU {
        CPU { cycles: 0, x_register: 1, listener}
    }

    fn increment_cycles(&mut self) {
        self.cycles += 1;
        if self.listener.is_speak_cycle(self.cycles) {
            self.speak();
        }
    }

    fn noop(&mut self) {
        self.increment_cycles();
    }

    fn addx(&mut self, x: i32) {
        self.increment_cycles();
        self.increment_cycles();
        self.x_register += x;
    }

    fn speak(&mut self) {
        self.listener.total += self.x_register * self.cycles as i32;
        println!("{}", self.cycles - self.listener.start);
        println!("Cycles: {}, Added: {}", self.cycles, self.x_register * self.cycles as i32);
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
    let contents = utils::read_file().unwrap();
    let listener = CycleListener::new(20, 40);
    let mut cpu = CPU::new(listener);

    for line in contents.lines() {
        cpu.read_instruction(line);
    }

    println!("Part 1: {}", cpu.listener.total);
}
