
const TEST_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
const LINE: [&str; 4] = ["#", "#", "#", "#"];
const PLUS: [[&str; 3]; 3] = [
    [".", "#", "."],
    ["#", "#", "#"],
    [".", "#", "."],
];
const L_SHAPE: [[&str; 3]; 3] = [
    ["#", "#", "#"],
    [".", ".", "#"],
    [".", ".", "#"],
];
const THREE_HEIGHT_EMPTY: [[&str; 7]; 3] = [
    [".",".",".",".",".",".",".",],
    [".",".",".",".",".",".",".",],
    [".",".",".",".",".",".",".",],
];
const VERTICAL_LINE: [[&str; 1]; 4] = [
    ["#"],
    ["#"],
    ["#"],
    ["#"],
];
const SQUARE: [[&str; 2]; 2] = [
    ["#", "#"],
    ["#", "#"],
];

#[derive(Debug, Clone)]
enum Rocks {
    HorizontalLine,
    Plus,
    LShape,
    VerticalLine,
    Square
}

impl Rocks {
    fn get_width(&self) -> usize {
        match *self {
            Rocks::HorizontalLine => 4,
            Rocks::VerticalLine => 1,
            Rocks::Plus | Rocks::LShape => 3,
            Rocks::Square => 2,
        }
    }

    fn get_height(&self) -> usize {
        match *self {
            Rocks::HorizontalLine => 1,
            Rocks::Plus | Rocks::LShape => 3,
            Rocks::VerticalLine => 4,
            Rocks::Square => 2,
        }
    }
    
    fn get_rock_shape(&self) -> Vec<Vec<&str>> {
        match *self {
            Rocks::HorizontalLine => vec![LINE.to_vec()],
            Rocks::Plus => PLUS.iter().map(|h| h.to_vec()).collect::<Vec<Vec<&str>>>(),
            Rocks::LShape => L_SHAPE.iter().map(|h| h.to_vec()).collect::<Vec<Vec<&str>>>(),
            Rocks::VerticalLine => VERTICAL_LINE.iter().map(|h| h.to_vec()).collect::<Vec<Vec<&str>>>(),
            Rocks::Square => SQUARE.iter().map(|h| h.to_vec()).collect::<Vec<Vec<&str>>>(),
        }
    }
}
/*
    Plus appears at (2, n) where n is current height
*/
pub fn solve_day_17() {
    let mut grid: Vec<Vec<&str>> = THREE_HEIGHT_EMPTY.iter().map(|h| h.to_vec()).collect::<Vec<Vec<&str>>>();
    // print_grid(&grid);
    let mut rock_count = 0;
    let mut gases_iter = TEST_INPUT.chars().into_iter().cycle();
    let rocks = vec![Rocks::HorizontalLine, Rocks::Plus, Rocks::LShape, Rocks::VerticalLine, Rocks::Square];
    let mut rocks_iter = rocks.iter().cycle();
    while rock_count <= 4 {
        // Start falling down loop
        // First get the height of the grid, this is where the rock will start
        let mut grid_height = grid.len();
        // Current rock falling
        let curr_rock = rocks_iter.next().unwrap();
        let mut width_range: (usize, usize) = (2, 1+curr_rock.get_width());
        'falling: loop {
            // Then jet moves the rock
            let jet_move = gases_iter.next().unwrap();
            println!("width range for {:?} is: {:?} and jet move is: {} and grid height is currently: {}", curr_rock, width_range, jet_move, grid_height);
            match jet_move {
                '>' => {
                    if width_range.1 < 6 {
                        width_range.1 += 1;
                        width_range.0 += 1;
                    }
                },
                '<' => {
                    if width_range.0 > 0 {
                        width_range.0 -= 1;
                        width_range.1 -= 1;
                    }
                },
                _ => unreachable!()
            }
            // Check to see if there is a rock/floor right under the falling rock
            // Need to check under each width index in width range
            // If there is, leave rock there and break out of loop
            for r in width_range.0..=width_range.1 {
                if grid_height == 0 || grid[grid_height-1][r] == "#" {
                    // Place rock
                    // println!("grid_height: {}, width range: {:?}", grid_height, width_range);
                    // for i in curr_rock.get_rock_shape() {
                    //     for c in i.iter().enumerate() {
                    //         grid[width_range.0][c.0] = *c.1;
                    //     }
                    // }
                    break 'falling;
                }
            }

            // else rock falls down
            grid_height -= 1;
        } 
        for (i, cols) in curr_rock.get_rock_shape().iter().enumerate() {
            for c in cols.iter().enumerate() {
                grid[grid_height + i][width_range.0 + c.0] = *c.1;
            }
        }
        // Add three empty spaces for next fall
       
        // Add three empty Spaces above ground or highest rock
        let height = get_height_of_rocks(&grid);
        println!("get_height_of_rocks: {}", height);
        for _ in height..height+1 {
            grid.push([".",".",".",".",".",".",".",].to_vec());
        }
        print_grid(&grid);
        println!("\n\n");
        // Rock has fallen, increase rock_count for next iteration
        rock_count += 1;
    }
}

fn get_height_of_rocks(grid: &Vec<Vec<&str>>) -> usize {
    for (i, row) in grid.iter().enumerate() {
        println!("{} -> {:?}", i, row);
        if row.iter().any(|x| *x == ".") && i != 0{
            // if the entire row contains '.' then the last row had at least one '#'
            return i - 1
        }
    }
    0
}

fn print_grid(grid: &Vec<Vec<&str>>) {
    for i in grid.iter().rev() {
        for j in i {
            print!("{}", j);
        }
        println!()
    }
}