use std::collections::{VecDeque, HashSet};

#[derive(Debug, Clone)]
struct Point {
    character: String,
    position: (u32, u32),
    parent: Option<(u32, u32)>
}

const INPUT: &str = include_str!("test_input.txt");

fn find_item(grid: &Vec<Vec<Point>>, item: String) -> Option<(u32, u32)> {
    for (i, row) in grid.iter().enumerate() {
        for (j, column) in row.iter().enumerate() {
            if *column.character == item {
                return Some(column.position);
            }
        }
    }
    None
}

fn get_adjacent_items(grid: &Vec<Vec<Point>>, vertex: (u32, u32)) -> Vec<(u32, u32)> {
    let mut res: Vec<(u32, u32)> = vec![];
    // Top
    if let Some(row) = grid.get(vertex.0 as usize) {
        if let Some(item) = row.get((vertex.1 + 1) as usize) {
            if 
                grid[vertex.0 as usize][vertex.1 as usize].character.as_bytes()[0] + 1 == item.character.as_bytes()[0] ||
                grid[vertex.0 as usize][vertex.1 as usize].character.as_bytes()[0] >= item.character.as_bytes()[0] || // Vertex is bigger than item
                grid[vertex.0 as usize][vertex.1 as usize].character == "S".to_string() ||
                (grid[vertex.0 as usize][vertex.1 as usize].character == "E".to_string() && item.character == "z")
            {
                // Item is one bigger than current vertex
                res.push((vertex.0, vertex.1 + 1));
            }
        }
    };
    // Left
    if vertex.0 != 0 {
        if let Some(row) = grid.get((vertex.0 - 1) as usize) {
            if let Some(item) = row.get((vertex.1) as usize) {
                if 
                    grid[vertex.0 as usize][vertex.1 as usize].character.as_bytes()[0] + 1 == item.character.as_bytes()[0] ||
                    grid[vertex.0 as usize][vertex.1 as usize].character.as_bytes()[0] >= item.character.as_bytes()[0] || // Vertex is bigger than item
                    grid[vertex.0 as usize][vertex.1 as usize].character == "S".to_string() ||
                    (grid[vertex.0 as usize][vertex.1 as usize].character == "E".to_string() && item.character == "z")
                {
                    res.push((vertex.0 - 1, vertex.1));
                }
            }
        }
    }
    // Right
    if let Some(row) = grid.get((vertex.0 + 1) as usize) {
        if let Some(item) = row.get(vertex.1 as usize) {
            if 
                grid[vertex.0 as usize][vertex.1 as usize].character.as_bytes()[0] + 1 == item.character.as_bytes()[0] ||
                grid[vertex.0 as usize][vertex.1 as usize].character.as_bytes()[0] >= item.character.as_bytes()[0] || // Vertex is bigger than item
                grid[vertex.0 as usize][vertex.1 as usize].character == "S".to_string() ||
                (grid[vertex.0 as usize][vertex.1 as usize].character == "E".to_string() && item.character == "z")
            {
                res.push((vertex.0 + 1, vertex.1));
            }
        }
    };
    // Bottom
    if vertex.1 != 0 {
        if let Some(row) = grid.get(vertex.0 as usize) {
            if let Some(item) = row.get((vertex.1 - 1) as usize) {
                if 
                    grid[vertex.0 as usize][vertex.1 as usize].character.as_bytes()[0] + 1 == item.character.as_bytes()[0] ||
                    grid[vertex.0 as usize][vertex.1 as usize].character.as_bytes()[0] >= item.character.as_bytes()[0] || // Vertex is bigger than item
                    grid[vertex.0 as usize][vertex.1 as usize].character == "S".to_string() ||
                    (grid[vertex.0 as usize][vertex.1 as usize].character == "E".to_string() && item.character == "z")
                {
                    res.push((vertex.0, vertex.1 - 1));
                }
            }
        }
    }

    res
}

pub fn solve_day_twelve() {
    let mut grid: Vec<Vec<Point>> = vec![];
    let mut row_count = 0;
    let mut column_count = 0;
    for line in INPUT.lines() {
        let mut row: Vec<Point> = vec![];
        for char in line.chars() {
            row.push(Point { character: char.to_string(), position: (column_count, row_count), parent: None});
            column_count += 1;
        }
        grid.push(row);
        row_count += 1;
        column_count = 0;
    }
    println!("Grid: {:#?}", grid);
    let start = find_item(&grid, "S".to_string()).unwrap();
    println!("Adj to start: {:?}", get_adjacent_items(&grid, start));
    let mut queue = VecDeque::new();
    queue.push_front(start);
    let mut visited = HashSet::new();
    let mut count = 1;
    // let mut found_counts = Vec::new();
    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();
        // println!("Current v: {:?} which is letter: {}", v, grid[v.0 as usize][v.1 as usize].character);
        count += 1;
        
        let adj = get_adjacent_items(&grid, v);
        for u in adj {
            if !visited.contains(&u) {
                grid[u.0 as usize][u.1 as usize].parent = Some(v);
                visited.insert(u);
                queue.push_back(u);
            }
        }
    }
    println!("Grid: {:#?}", grid);

    // let end = find_item(&grid, "E".to_string()).unwrap();
    // println!("{:?}", end);
    // let end = &grid[end.0 as usize][end.1 as usize];
    // println!("parent: {:?}", grid[end.parent.unwrap().0 as usize][end.parent.unwrap().1 as usize]);

    // println!("visited Array: {:?}", visited);
}