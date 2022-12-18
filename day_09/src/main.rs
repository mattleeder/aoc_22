use common::utils;
use std::collections::HashSet;
use std::cell::RefCell;
use std::rc::Rc;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum KnotType {
    Head,
    Middle,
    Tail,
}

#[derive(Debug)]
struct Rope {
    position: RefCell<[i16; 2]>,
    tail: Option<Box<Rope>>,
    visited: Rc<RefCell<HashSet<[i16; 2]>>>,
    knot: KnotType,
}

impl Rope {
    fn new(n: usize) -> Rope {
        let mut visited: HashSet<[i16; 2]> = HashSet::new();
        visited.insert([0, 0]);
        let visited = RefCell::new(visited);
        let visited = Rc::new(visited);
        
        let mut head = Rope {
            position: RefCell::new([0, 0]),
            tail: None,
            visited: Rc::clone(&visited),
            knot: KnotType::Head,
        };

        let mut v: Vec<Box<Rope>> = Vec::new();

        for _ in 1..n {
            v.push(
                Box::new(Rope {
                    position: RefCell::new([0, 0]),
                    tail: None,
                    visited: Rc::clone(&visited),
                    knot: KnotType::Middle,
                })
            );
        }

        v.push(Box::new(Rope {
            position: RefCell::new([0, 0]),
            tail: None,
            visited: Rc::clone(&visited),
            knot: KnotType::Tail,
        }));

        while v.len() > 1 {
            let length = v.len();
            let tail = Some(v.pop().unwrap());
            v[length - 2].tail = tail; 
        }

        let tail = Some(v.pop().unwrap());
        head.tail = tail;

        head
    }

    fn is_touching_parent(&self, parent_position: [i16; 2]) -> bool{
        let x = (parent_position[0] - self.position.borrow()[0]).abs();
        let y = (parent_position[1] - self.position.borrow()[1]).abs();

        if x <= 1 && y <= 1 {
            return true
        }

        false
    }

    fn move_child(&self, parent_position: [i16; 2]) {
        if matches!(self.knot, KnotType::Head) {
            println!("Cannot call move_tail on a head");
            return
        }
        let x = parent_position[0] - self.position.borrow()[0];
        let y = parent_position[1] - self.position.borrow()[1];

        let dir = [x.clamp(-1, 1), y.clamp(-1, 1)];

        self.position.borrow_mut()[0] += dir[0];
        self.position.borrow_mut()[1] += dir[1];

        if matches!(self.knot, KnotType::Tail) {
            self.visited.as_ref().borrow_mut().insert(self.position.borrow().clone());
        }

        if self.tail.is_some() {
            if !self.tail.as_ref().unwrap().is_touching_parent(self.position.borrow().clone()) {
                self.tail.as_ref().unwrap().move_child(self.position.borrow().clone());
            }
        }
    }

    fn move_head(&mut self, dir: Direction, n: i16) {
        if !matches!(self.knot, KnotType::Head) {
            println!("Can only call move_head on a head");
            return
        }

        for _ in 0..n {
            match dir {
                Direction::Up => {self.position.borrow_mut()[1] += 1},
                Direction::Down => {self.position.borrow_mut()[1] -= 1},
                Direction::Right => {self.position.borrow_mut()[0] += 1},
                Direction::Left => {self.position.borrow_mut()[0] -= 1},
            }
            if self.tail.is_some() {
                if !self.tail.as_ref().unwrap().is_touching_parent(self.position.borrow().clone()) {
                    self.tail.as_ref().unwrap().move_child(self.position.borrow().clone());
                }
            }
        }
    }
}

fn main() {
    let contents = utils::read_file().unwrap();
    let mut rope = Rope::new(9);

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

    let tot = rope.visited.borrow().len();

    println!("Part 1: {}", tot);
}
