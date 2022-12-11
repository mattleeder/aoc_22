use common::utils;

#[derive(Debug)]
struct Containers {
    stacks: Vec<Vec<char>>,
}

impl Containers {

    fn new(n: usize) -> Containers {
        return Containers {
            stacks: std::iter::repeat(Vec::new()).take(n).collect::<Vec<_>>(),
        }
    }

    fn read_command(&mut self, cmd: &str) {
        let mut instructions = cmd.split(' ');
        instructions.next();
        let mut instructions = instructions.step_by(2);

        let amount = instructions.next().unwrap().parse::<u32>().unwrap();
        let origin = instructions.next().unwrap().parse::<usize>().unwrap() - 1;
        let destination = instructions.next().unwrap().parse::<usize>().unwrap() - 1;

        let mut index = 0;

        while index < amount {
            let container = self.stacks[origin].pop().expect("Cannot pop empty Vector");
            self.stacks[destination].push(container);
            index += 1;
        }
    }

}

fn main() {
    let binding = utils::read_file().unwrap();
    let mut contents = binding.lines();

    let first_line = contents.next().expect("Could not read first line");
    let num_stacks = (first_line.len() + 1) / 4;
    println!("There are {} stacks", num_stacks);

    let mut containers = Containers::new(num_stacks);


    let mut index = 0;
    let mut first_line = first_line.chars();
    // Consume first element
    first_line.next();

    for c in first_line.step_by(4) {
        if c != ' ' {
            containers.stacks[index].push(c);
        }
        index += 1;
    }

    for line in &mut contents {
        if &line[1..2] == "1" {
            break
        }
        let mut line = line.chars();
        let mut index = 0;
        // Consumes first element
        line.next();

        for c in line.step_by(4) {
            if c != ' ' {
                containers.stacks[index].push(c);
            }
            index += 1;
        }
    }

    // Reverse the order of each stack, first 
    // lines read in are actually on top
    
    for stack in containers.stacks.iter_mut() {
        stack.reverse();
    }
    // Consume blank line
    contents.next();

    println!("{:?}", containers.stacks);

    for line in contents {
        containers.read_command(line);
    }

    println!("{:?}", containers.stacks);

    for stack in containers.stacks.iter() {
        println!("{}", stack.last().unwrap());
    }
}
