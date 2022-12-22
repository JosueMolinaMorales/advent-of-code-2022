use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
struct Node<'a> {
    name: &'a str,
    operation: &'a str              // Holds the operation to happen, or number: eg. x * y or 4
}

pub fn solve_day_21() {
    let mut tree: HashMap<&str, Node> = HashMap::new();
    INPUT.lines().for_each(|line| {
        let mut line = line.trim().split(": ");
        let name = line.next().unwrap();
        tree.insert(name.clone(), Node { name, operation: line.next().unwrap() });
    });

    // Part 1
    let sum = traverse(&mut tree, "root");
    println!("{sum}");

    // Part 2
    let human_yell = 0;
}

fn traverse_part_2<'a>(tree: &HashMap<&str, Node>, curr_node: &'a str) {
    
}

fn traverse<'a>(tree: &HashMap<&str, Node>, curr_node: &'a str) -> isize {
    let node = tree.get(curr_node).unwrap();
    if let Some(num) = node.operation.parse::<isize>().ok() {
        return num
    } else {
        // Operation requires two other nodes
        let mut operations = node.operation.split(" ");
        let first_node = operations.next().unwrap();
        let op = operations.next().unwrap();
        let second_node = operations.next().unwrap();

        let first_node = traverse(tree, first_node);
        let second_node = traverse(tree, second_node);

        return match op {
            "+" => first_node + second_node,
            "-" => first_node - second_node,
            "*" => first_node * second_node,
            "/" => first_node / second_node,
            _ => unreachable!()
        }
    }
}