use std::collections::{HashMap, HashSet};

use common::utils;

struct BFS<'a> {
    queue: Vec<[i8; 2]>,
    visited: HashSet<[i8; 2]>,
    grid: &'a Vec<Vec<i8>>,
    count: u16,
    level_length: usize,
}

impl BFS<'_> {

    fn new(start: [i8; 2], grid: &Vec<Vec<i8>>) -> BFS {
        let mut visited = HashSet::from([start]);
        let rows = grid.len() as i8;
        let cols = grid[0].len() as i8;
        // Add the area around the grid to visited
        // to avoid needing to check if in bounds
        for i in 0..rows {
            visited.insert([i, -1]);
            visited.insert([i, cols]);
        }
        for i in 0..cols {
            visited.insert([-1, i]);
            visited.insert([rows, i]);
        }
        BFS {
            queue: vec![start],
            visited,
            grid,
            count: 0_u16,
            level_length: 1_usize,
        }
    }

    fn get_adjacent(&self, point: &[i8; 2]) -> Vec<[i8; 2]> {
        let mut adjacent: Vec<[i8; 2]> = Vec::new();
        // Remember: 0,0 is at top left
        let up = [point[0] - 1, point[1]];
        let down = [point[0] + 1, point[1]];
        let left = [point[0], point[1] - 1];
        let right = [point[0], point[1] + 1];

        if !self.visited.contains(&up) {
            adjacent.push(up);
        }
        if !self.visited.contains(&down) {
            adjacent.push(down);
        }
        if !self.visited.contains(&left) {
            adjacent.push(left);
        }
        if !self.visited.contains(&right) {
            adjacent.push(right);
        }
        adjacent
    }

    fn get_height(&self, point: &[i8; 2]) -> i8 {
        self.grid[point[0] as usize][point[1] as usize]
    }

}

impl Iterator for BFS<'_> {
    type Item = ([i8; 2], u16);

    fn next(&mut self) -> Option<Self::Item> {
        if self.level_length == 0 {
            self.count += 1;
            self.level_length = self.queue.len();
        }
        if let Some(point) = self.queue.pop() {
            // println!("Queue: {:?}", self.queue);
            let height = self.get_height(&point);
            for p in self.get_adjacent(&point).iter() {
                let p_height = self.get_height(p);
                if height - p_height >= -1 {
                    self.queue.insert(0, p.to_owned());
                    self.visited.insert(p.to_owned());
                }
            }
            self.level_length -= 1;
            return Some((point, self.count))
        }
        None
    }

}

fn main() {
    let contents = utils::read_file().unwrap();
    let keys = "abcdefghijklmnopqrstuvwxyz";
    let mut height_map: HashMap<char, i8> = HashMap::new();
    for (i, k) in keys.chars().enumerate() {
        height_map.insert(k, i as i8);
    }
    height_map.insert('S', 0);
    height_map.insert('E', 25);
    let mut grid: Vec<Vec<i8>> = Vec::new();
    let mut start: [i8; 2] = [-1, -1];
    let mut end: [i8; 2] = [-1, -1];

    for (i, line) in contents.lines().enumerate() {
        let mut row: Vec<i8> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                start = [i as i8, j as i8];
            } else if c == 'E' {
                end = [i as i8, j as i8];
            }
            row.push(height_map.get(&c).expect("Could not find character in height_map").to_owned());
        }
        grid.push(row);
    }

    let bfs = BFS::new(start, &grid);

    for (point, count) in bfs {
        if point == end {
            println!("Part 1: {}", count);
        }
    }
    
    println!("Start: {:?}", start);
    println!("End: {:?}", end);
}
