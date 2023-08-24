use regex::Regex;
use std::fs::read_to_string;

pub fn top_crates(filename: &str, in_order: bool) -> String {
    let mut stacks = parse_stacks(filename);
    apply_movements(&mut stacks, &movements_string(filename), in_order);
    stacks.top_items()
}

fn parse_stacks(filename: &str) -> Vec<Stack> {
    let stack_str = stack_string(filename);
    let mut stacks = create_stacks(&stack_str);

    // read in reverse to push items from bottom up onto stack
    // skip the first line that contains the stack numbers
    for line in stack_str.lines().rev().skip(1) {
        for i in 0..stacks.len() {
            if let Some(item) = item(line, i) {
                stacks[i].items.push(item);
            }
        }
    }

    stacks
}

fn apply_movements(stacks: &mut Vec<Stack>, mov_str: &String, in_order: bool) {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for line in mov_str.lines() {
        if let Some(cap) = re.captures(line) {
            // since we got a match we'll assume that unwrap always works
            let n = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let from = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let to = cap.get(3).unwrap().as_str().parse::<usize>().unwrap();

            // zero based vector indices, input file starts at 1
            if in_order {
                stacks.move_n_in_order(n, from - 1, to - 1);
            } else {
                stacks.move_n(n, from - 1, to - 1);
            }
        }
    }
}

fn item(line: &str, column: usize) -> Option<char> {
    // lines in input have following format:
    // [A] [B] [C] ...
    // the relevant values appear every 4 characters starting at 1
    match line.chars().nth(1 + column * 4) {
        Some(c) if c.is_alphabetic() => Some(c),
        _ => None,
    }
}

fn create_stacks(str: &String) -> Vec<Stack> {
    let mut stacks = vec![];
    for _ in 0..stack_count(str) {
        stacks.push(Stack::create());
    }
    stacks
}

fn stack_count(str: &String) -> usize {
    // get the line with the stack numbers and extract its last value
    // (assuming they are always in order)
    str.lines()
        .rev()
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap()
}

fn stack_string(filename: &str) -> String {
    let content = read_to_string(filename).unwrap_or(String::from(""));
    let mut stack_str = String::from("");
    // read until empty line after which movement instructions follow
    for line in content.lines() {
        if line.is_empty() {
            break;
        }
        stack_str += &format!("{}\n", line);
    }
    stack_str
}

fn movements_string(filename: &str) -> String {
    let content = read_to_string(filename).unwrap_or(String::from(""));
    let mut mov_str = String::from("");
    // read in reverse until empty line after which the stack layout follows
    for line in content.lines().rev() {
        if line.is_empty() {
            break;
        }
        mov_str += &format!("{}\n", line);
    }

    // reverse once again to get movements in correct order
    mov_str
        .lines()
        .rev()
        .fold(String::from(""), |str, line| str + line + "\n")
}

struct Stack {
    items: Vec<char>,
}

impl Stack {
    fn create() -> Stack {
        Stack { items: vec![] }
    }
}

trait StackMovement {
    fn move_n(&mut self, n: usize, from: usize, to: usize);
    fn move_n_in_order(&mut self, n: usize, from: usize, to: usize);
}

impl StackMovement for Vec<Stack> {
    fn move_n(&mut self, n: usize, from: usize, to: usize) {
        for _ in 0..n {
            if let Some(item) = self[from].items.pop() {
                self[to].items.push(item);
            }
        }
    }

    fn move_n_in_order(&mut self, n: usize, from: usize, to: usize) {
        // store items in temporary stack (reverse order)
        let mut temp = Stack::create();

        for _ in 0..n {
            if let Some(item) = self[from].items.pop() {
                temp.items.push(item);
            }
        }

        // reverse temporary stack to get items back in original order
        while let Some(item) = temp.items.pop() {
            self[to].items.push(item);
        }
    }
}

trait StackInspect {
    fn top_items(&self) -> String;
    fn print(&self);
}

impl StackInspect for Vec<Stack> {
    fn top_items(&self) -> String {
        self.iter().fold(String::from(""), |mut str, stack| {
            if let Some(item) = stack.items.last() {
                str.push(*item);
            } else {
                str.push(' ');
            }
            str
        })
    }

    // print a visual representation of the stacks
    fn print(&self) {
        let max_len = self.iter().map(|stack| stack.items.len()).max().unwrap();
        for i in (0..max_len).rev() {
            for stack in self {
                if stack.items.len() > i {
                    print!("[{}] ", stack.items[i]);
                } else {
                    print!("    ");
                }
            }
            println!("");
        }
    }
}
