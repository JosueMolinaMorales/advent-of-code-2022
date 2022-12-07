use std::{fs, collections::BTreeMap};

const TEST_INPUT: &str = "./input7.txt";
const INPUT_FILE: &str = "./inputs/day_7_input.txt";

#[derive(Debug)]
struct File {
    size: u32,
    name: String
}

#[derive(Debug)]
struct Directory {
    name: String,
    size: Option<u32>,
    parent: Option<Box<Directory>>,
    files: Option<Vec<File>>
}

#[derive(Debug)]
enum Commands {
    CD,
    LS
}

pub fn solve_day_seven() {
    let input = fs::read_to_string(TEST_INPUT).unwrap();
    part_one(input);
}

fn part_one(input: String) {
    let mut current_dir = "".to_string();
    let mut last_command: Commands = Commands::CD;
    let mut directory_structure: BTreeMap<String, Directory> = BTreeMap::new(); // String will be name of directory
    let mut stack: Vec<String> = Vec::new(); // Stores the history of directories. Used for ..

    for output in input.split("\n") {
        match output.chars().nth(0).unwrap() {
            '$' => {
                // Executable action
                let parse = output.split(" ").map(|f| f.trim()).collect::<Vec<&str>>();
                match parse[1] {
                    "cd" => {
                        last_command = Commands::CD;
                        // CD options are -> .. or {name}
                        match parse[2] {
                            ".." => {
                                // Go back
                                stack.pop();
                                current_dir = stack.last().unwrap().to_owned();
                            },
                            dir_name => {
                                if current_dir == "" {
                                    // previous directory
                                    // Inserts the directory into the directory structure, if it is already in there, None is returned
                                    directory_structure.insert(dir_name.to_string(), Directory { 
                                        name: dir_name.to_string(), 
                                        size: None, 
                                        sub_directories: None, 
                                        files: None 
                                    });
                                } else {
                                    // Inside a current directory, loop through stack and get the Directory Object
                                    for dir_name_from_stack in stack.iter() {
                                        let dir = directory_structure.get_mut(dir_name_from_stack).unwrap();
                                        if dir.name == current_dir {
                                            // Within the current dir, add the directory to 
                                            match dir.sub_directories.as_mut() {
                                                None => dir.sub_directories = Some(vec![Box::new(Directory{ name: dir_name.to_string(), size: None, sub_directories: None, files: None})]),
                                                Some(sub_dirs) => {
                                                    if sub_dirs.iter().find(|d| d.name == dir_name.to_string()).is_none() {
                                                        // Directory not in subdir list, add it
                                                        sub_dirs.push(Box::new(Directory { name: dir_name.to_string(), size: None, sub_directories: None, files: None}))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                // Insert the directory into the stack
                                stack.push(dir_name.to_string());
                                // Set the current directory
                                current_dir = dir_name.to_string();
                            }
                        }
                    },
                    "ls" => {
                        last_command = Commands::LS
                    },
                    _ => unreachable!()
                }
            },
            _ => {
                // Line is ls output
                let output_line = output.split(" ").collect::<Vec<&str>>();
                match output_line[0] {
                    "dir" => {
                        // This is a directory
                        let name_of_dir = output_line[1].trim();
                        for dir_name in stack.iter() {
                            println!("{}", dir_name);
                            let dir = directory_structure.get_mut(dir_name).unwrap();
                            if dir.name == current_dir {
                                // Within the current dir, add the directory to 
                                match dir.sub_directories.as_mut() {
                                    None => dir.sub_directories = Some(vec![Box::new(Directory{ name: name_of_dir.to_string(), size: None, sub_directories: None, files: None})]),
                                    Some(sub_dirs) => {
                                        if sub_dirs.iter().find(|d| d.name == name_of_dir.to_string()).is_none() {
                                            // Directory not in subdir list, add it
                                            sub_dirs.push(Box::new(Directory { name: name_of_dir.to_string(), size: None, sub_directories: None, files: None}))
                                        }
                                    }
                                }
                            }
                        }
                    },
                    size => {
                        // This is a file
                        for dir_name in stack.iter() {
                            let dir = directory_structure.get_mut(dir_name).unwrap();
                            if dir.name == current_dir {
                                // Within the current dir, add the file to directory
                                let file_name = output_line[1].to_string();
                                match dir.files.as_mut() {
                                    None => {
                                        dir.files = Some(vec![File{name: file_name, size: size.parse().unwrap()}])
                                    },
                                    Some(files) => {
                                        // Files exists
                                        if files.iter().find(|f| f.name == file_name).is_none() {
                                            // file does not exist in file vec
                                            files.push(File { size: size.parse().unwrap(), name: file_name})
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
            }
        }
    }

    println!("After loop: {:#?}", directory_structure);
}