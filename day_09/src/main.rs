use common::utils;
use std::collections::HashSet;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Rope {
    head: [i16; 2],
    tail: [i16; 2],
    visited: HashSet<[i16; 2]>,
}

impl Rope {
    fn new(head: [i16; 2], tail: [i16; 2]) -> Rope {
        let mut visited = HashSet::new();
        visited.insert(tail);
        Rope {
            head,
            tail,
            visited,
        }
    }

    fn tail_is_touching_head(&self) -> bool{
        let x = (self.head[0] - self.tail[0]).abs();
        let y = (self.head[1] - self.tail[1]).abs();

        if x <= 1 && y <= 1 {
            return true
        }

        false
    }

    fn move_tail(&mut self) {
        let x = self.head[0] - self.tail[0];
        let y = self.head[1] - self.tail[1];

        let dir = [x.clamp(-1, 1), y.clamp(-1, 1)];

        self.tail[0] += dir[0];
        self.tail[1] += dir[1];

        self.visited.insert(self.tail);

    }

    fn move_head(&mut self, dir: Direction, n: i16) {
        for _ in 0..n {
            match dir {
                Direction::Up => {self.head[1] += 1},
                Direction::Down => {self.head[1] -= 1},
                Direction::Right => {self.head[0] += 1},
                Direction::Left => {self.head[0] -= 1},
            }
            if !self.tail_is_touching_head() {
                self.move_tail();
            }
        }

    }
}

fn main() {
    let contents = utils::read_file().unwrap();
    let mut rope = Rope::new([0, 0], [0, 0]);

    for line in contents.lines() {
        let v: Vec<&str> = line.split(' ').collect();

        let dir = match v[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "R" => Direction::Right,
            "L" => Direction::Left,
            _ => panic!("Unexpected direction"),
        };

        let n = v[1].parse::<i16>().unwrap();

        rope.move_head(dir, n);
    }

    let tot = rope.visited.len();

    println!("Part 1: {}", tot);
}
