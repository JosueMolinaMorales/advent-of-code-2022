use std::{fs, collections::HashMap };

// const TEST_INPUT: &str = "./input7.txt";
const INPUT_FILE: &str = "./inputs/day_7_input.txt";

pub fn solve_day_seven() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    part_one(input);
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    size: u32,
    parent: Option<String>,
    children: Vec<String>
}


fn part_one(input: String) {
    let mut directory_structure: HashMap<String, Directory> = HashMap::new();
    let mut stack: Vec<String> = Vec::new(); // Stores the history of directories. 

    for line in input.split("\n") {
        match line.chars().nth(0).unwrap() {
            '$' => {
                let exec = line.split(" ").map(|s| s.trim()).collect::<Vec<&str>>();
                if exec[1] != "cd" {
                    continue
                }
                let dir_name = exec[2].to_string();
                if dir_name == ".." {
                    stack.pop();
                    continue;
                }
                if dir_name == "/" {
                    directory_structure.insert("/".to_string(), Directory { name: "/".to_string(), size: 0, parent: None, children: Vec::new() });
                }

                stack.push(dir_name);
            },
            _ => {
                // ls output
                let output = line.split(" ").map(|s| s.trim()).collect::<Vec<&str>>();
                match output[0] {
                    "dir" => {
                        let dir_name = output[1].to_string();  
                        directory_structure
                        .entry(format!("{}/{}", stack.join("/"), dir_name.clone()))
                        .and_modify(|dir| {
                            match dir.parent {
                                None => {
                                    dir.parent = Some(stack.join("/"))
                                },
                                Some(_) => {}
                            }
                        })
                        .or_insert(Directory {
                            name: format!("{}/{}", stack.join("/"), dir_name.clone()),
                            size: 0,
                            parent: Some(stack.join("/")),
                            children: Vec::new()
                        });

                        // add dir as a child of the current directory
                        directory_structure
                            .entry(stack.join("/"))
                            .and_modify(|dir| {
                                dir.children.push(format!("{}/{}", stack.join("/"), dir_name.clone()));
                            });
                        
                    },
                    size => {
                        let size = size.parse::<u32>().unwrap(); 
                        directory_structure
                            .entry(stack.join("/"))
                            .and_modify(|dir| {
                                dir.size += size;
                            });
                    }
                }
            }
        }
    }

    for dir in directory_structure.clone().values() {
        let children = dir.children.clone();
        for child in children {
            let child_size = directory_structure.get(&child).unwrap().size;
            directory_structure
                .entry(dir.name.clone())
                .and_modify(|d| {
                    d.size += child_size 
                });
        }   
    }
    println!("{:#?}", directory_structure);
    let res = directory_structure.clone().iter()
    .map(|dir| {
        dir.1.size
    })
    .filter(|size| *size < 100000)
    .sum::<u32>();
    println!("part 1 res: {}", res);

    let total_space_available = 70000000;
    let required_unused_space = 30000000;
    let total_space_used: u32 = directory_structure.iter().map(|d| d.1.size).sum();
    let current_unused_space = total_space_available - total_space_used;

    let mut part_two = directory_structure.iter()
    .map(|dir| {
        dir.1.size
    })
    .filter(|size| {
        *size > required_unused_space - current_unused_space
    })
    .collect::<Vec<u32>>();
    part_two.sort();
    println!("{:?}", part_two);
    println!("part 2: {:?}", part_two.iter().min().unwrap());
    
}
