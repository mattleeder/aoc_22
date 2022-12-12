use common::utils;

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<i8>>,
    representation: Vec<Vec<i8>>,
}

impl Grid {

    fn new() -> Grid {
        return Grid {
            data: Vec::new(),
            representation: Vec::new(),
        }
    }

    fn read_row(&mut self, row: &str) {
        self.data.push(row.chars().map(|x| x.to_digit(10).unwrap() as i8).collect());
        self.representation.push(vec![0; row.len()]);
    }

    fn init_representation(&mut self) {
        let length = self.representation[0].len();
        self.representation[0] = vec![1; length];
        self.representation[length - 1] = vec![1; length];

        for row in self.representation.iter_mut() {
            let length = row.len();
            row[0] = 1;
            row[length - 1] = 1;
        }

    }

    fn visible_trees(&self) -> u32 {
        let mut tot = 0_u32;
        for row in self.representation.iter() {
            tot += row.iter().sum::<i8>() as u32;
        }

        tot
    }

    fn scenic_score(&self, row: usize, col: usize) -> u32 {
        // Calculates the scenic score for the tree at row, col
        let up = self.trees_visible(row, col, Direction::Up);
        let down = self.trees_visible(row, col, Direction::Down);
        let left = self.trees_visible(row, col, Direction::Left);
        let right = self.trees_visible(row, col, Direction::Right);

        up * down * left * right
    }

    fn trees_visible(&self, mut row: usize, mut col: usize, dir: Direction) -> u32 {
        let mut visible = 0;
        let height = self.data[row][col];
        let rows = self.data.len();
        let cols = self.data[0].len();

        match dir {
            Direction::Left => {
                while col > 0 {
                    visible += 1;
                    if self.data[row][col - 1] >= height {
                        break;
                    } else {
                        col -= 1;
                    }
                }
            },
            Direction::Right => {
                while col < cols - 1 {
                    visible += 1;
                    if self.data[row][col + 1] >= height {
                        break;
                    } else {
                        col += 1;
                    }
                }
            },
            Direction::Down => {
                while row < rows - 1 {
                    visible += 1;
                    if self.data[row + 1][col] >= height {
                        break;
                    } else {
                        row += 1;
                    }
                }
            },
            Direction::Up => {
                while row > 0 {
                    visible += 1;
                    if self.data[row - 1][col] >= height {
                        break;
                    } else {
                        row -= 1;
                    }
                }
            }
        }

        visible
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct GridReader {}

impl GridReader {

    pub fn new() -> GridReader {
        return GridReader {}
    }

    pub fn read_grid(&self, grid: &mut Grid, dir: Direction) {
        match dir {
            Direction::Up => self.read_up(grid),
            Direction::Down => self.read_down(grid),
            Direction::Left => self.read_left(grid),
            Direction::Right => self.read_right(grid),
        }
    }

    fn read_right(&self, grid: &mut Grid) {

        for (i, row_data) in grid.data.iter().enumerate() {
            let mut tallest = -1_i8;
            'inner: for (j, num) in row_data.iter().enumerate() {
                if num > &tallest {
                    tallest = *num;
                    grid.representation[i][j] = 1;
                    if tallest == 9 {
                        break 'inner;
                    }
                    
                }
            }
        }
    }

    fn read_left(&self, grid: &mut Grid) {

        for (i, row_data) in grid.data.iter().enumerate() {
            let mut tallest = -1_i8;
            'inner: for (j, num) in row_data.iter().enumerate().rev() {
                if num > &tallest {
                    tallest = *num;
                    grid.representation[i][j] = 1;
                    if tallest == 9 {
                        break 'inner;
                    }
                    
                }
            }
        }
    }

    fn read_down(&self, grid: &mut Grid) {

        for j in 0..grid.data[0].len() {
            let mut tallest = -1_i8;
            'inner: for (i, row_data) in grid.data.iter().enumerate() {
                if row_data[j] > tallest {
                    tallest = row_data[j];
                    grid.representation[i][j] = 1;
                    if tallest == 9 {
                        break 'inner;
                    }
                }
            }
        }
    }

    fn read_up(&self, grid: &mut Grid) {

        for j in 0..grid.data[0].len() {
            let mut tallest = -1_i8;
            'inner: for (i, row_data) in grid.data.iter().enumerate().rev() {
                if row_data[j] > tallest {
                    tallest = row_data[j];
                    grid.representation[i][j] = 1;
                    if tallest == 9 {
                        break 'inner;
                    }
                }
            }
        }
    }
}

fn main() {
    let binding = utils::read_file().unwrap();

    let mut grid = Grid::new();

    for row in binding.lines() {
        grid.read_row(row);
    }

    grid.init_representation();

    let reader = GridReader::new();
    
    reader.read_grid(&mut grid, Direction::Left);
    reader.read_grid(&mut grid, Direction::Right);
    reader.read_grid(&mut grid, Direction::Up);
    reader.read_grid(&mut grid, Direction::Down);

   let res = grid.visible_trees();

   println!("Visible trees: {res}");

   let mut high_score = 0_u32;

   for row in 0..grid.data.len() {
       for col in 0..grid.data[0].len() {
           let score = grid.scenic_score(row, col);
           if score > high_score {
               high_score = score;
           }
       }
   }

   println!("Most scenic tree: {high_score}");
}
