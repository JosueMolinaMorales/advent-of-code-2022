use std::{fs, collections::{BTreeMap, HashMap}};

const TEST_INPUT: &str = "./input7.txt";
const INPUT_FILE: &str = "./inputs/day_7_input.txt";

pub fn solve_day_seven() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    part_one(input);
}

#[derive(Debug)]
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

                // Check to see if the dir name is already in struct
                let (parent, parent_size) = match directory_structure.get(stack.last().unwrap_or(&"".to_string())) {
                    Some(parent) => (Some(parent.name.clone()), parent.size),
                    None => (None, 0)
                };

                directory_structure
                .entry(dir_name.clone())
                .or_insert(Directory { 
                    name: dir_name.clone(),
                    size: parent_size, 
                    parent,
                    children: Vec::new()
                });

                stack.push(dir_name);
            },
            _ => {
                // ls output
                let output = line.split(" ").map(|s| s.trim()).collect::<Vec<&str>>();
                match output[0] {
                    "dir" => {
                        let dir_name = output[1].to_string();
                        directory_structure
                        .entry(dir_name.clone())
                        .and_modify(|dir| {
                            match dir.parent {
                                None => {
                                    dir.parent = Some(stack.last().unwrap().clone())
                                },
                                Some(_) => {}
                            }
                        })
                        .or_insert(Directory {
                            name: dir_name.clone(),
                            size: 0,
                            parent: Some(stack.last().unwrap().clone()),
                            children: Vec::new()
                        });

                        let curr_dir = stack.last().unwrap().clone();
                        // add dir as a child of the current directory
                        directory_structure
                            .entry(curr_dir.clone())
                            .and_modify(|dir| {
                                dir.children.push(dir_name.clone());
                            });
                        
                    },
                    size => {
                        let curr_dir = stack.last().unwrap().clone();
                        let size = size.parse::<u32>().unwrap(); 
                        directory_structure
                            .entry(curr_dir.to_string())
                            .and_modify(|dir| {
                                dir.size += size;
                            });
                        let mut parent_dir = directory_structure.get(&curr_dir).unwrap().parent.clone();
                        while let Some(parent) = parent_dir.clone() {
                            directory_structure.entry(parent.clone())
                            .and_modify(|dir| {
                                dir.size += size;
                                parent_dir = dir.parent.clone(); 
                            });
                        }
                    }
                }
            }
        }
    }


    let res = directory_structure.iter()
    .map(|dir| {
        dir.1.size
    })
    .filter(|size| *size < 100000)
    .sum::<u32>();
    println!("part 1 res: {}", res);
}

// #[test]
// fn test_find_sub_dir() {
//     let mut sub_dir_tree = BTreeMap::new();
//     sub_dir_tree.insert("D".to_string(), Box::new(Directory {
//         name: "D".to_string(),
//         size: None,
//         sub_directories: BTreeMap::new(),
//         files: Vec::new(),
//     }));
//     let mut dir_tree = BTreeMap::new();
//     dir_tree.insert("A".to_string(), Box::new(Directory {
//         name: "A".to_string(),
//         size: None,
//         sub_directories: BTreeMap::new(),
//         files: Vec::new()
//     }));
//     dir_tree.insert("B".to_string(), Box::new(Directory {
//         name: "B".to_string(),
//         size: None,
//         sub_directories: BTreeMap::new(),
//         files: Vec::new()
//     }));
//     dir_tree.insert("C".to_string(),Box::new(Directory {
//         name: "C".to_string(),
//         size: None,
//         sub_directories: sub_dir_tree,
//         files: Vec::new()
//     }));
//     let dir_struct = Directory {
//         name: "/".to_string(),
//         size: None,
//         sub_directories: dir_tree,
//         files: Vec::new()
//     };

//     let found = dir_struct.find_subdirectory("D".to_string());
//     assert!(found.is_some());
//     assert_eq!(found.unwrap().name, "D".to_string())
// }