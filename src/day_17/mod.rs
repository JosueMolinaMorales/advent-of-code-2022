use std::collections::HashSet;


// const TEST_INPUT: &str =  // "><<<>><><<><><<<<<<<>><>>><<<>>>>>";
const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";//include_str!("day_17_input.txt");

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Gas {
    Right,
    Left
}
impl Gas {
    fn new(c: char) -> Gas {
        match c {
            '>' => Gas::Right,
            '<' => Gas::Left,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

    fn get_starting_positions(&self, h: usize) -> Vec<(usize, usize)> {
        match *self {
            Rocks::HorizontalLine => vec![
                (2, h),
                (3, h),
                (4, h),
                (5, h)
            ],
            Rocks::Plus => vec![
                (3, h), // Bottom
                (2, h+1),
                (3, h+1),
                (4, h+1),
                (3, h+2) // top
            ],
            Rocks::LShape => vec![
                (2, h),
                (3, h),
                (4, h),
                (4, h+1),
                (5, h+2)
            ],
            Rocks::VerticalLine => vec![
                (2, h),
                (2, h+1),
                (2, h+2),
                (2, h+3),
            ],
            Rocks::Square => vec![
                (2, h),
                (3, h),
                (2, h+1),
                (3, h+1)
            ],
        }
    }

    fn move_rock_right(&self, curr_pos: &mut Vec<(usize, usize)>, grid: &HashSet<(usize, usize)>) {
        // When moving rock right, need to check that its x pos <= 6, also checking for other rocks
        let mut can_move_right = true;
        for (x, y) in curr_pos.iter() {
            if *x + 1 == 7 || grid.contains(&(*x+1, *y)) {
                // if grid contains (x+1, y) then there is already a rock in that position and it cannot move.
                can_move_right = false;
                break;
            }
        }
        if can_move_right {
            for (x, _) in curr_pos {
                *x += 1;
            }
        }
    }

    fn move_rock_left(&self, curr_pos: &mut Vec<(usize, usize)>, grid: &HashSet<(usize, usize)>) {
        // When moving rock right, need to check that its x pos <= 6, also checking for other rocks
        let mut can_move_left = true;
        for (x, y) in curr_pos.iter() {
            if *x as isize - 1 < 0 || grid.contains(&(*x-1, *y)) {
                // if grid contains (x+1, y) then there is already a rock in that position and it cannot move.
                can_move_left = false;
                break;
            }
        }
        if can_move_left {
            for (x, _) in curr_pos {
                *x -= 1;
            }
        }
    }

    fn can_rock_move_down(&self, curr_pos: &Vec<(usize, usize)>, grid: &HashSet<(usize, usize)>) -> bool {
        let mut can_move_down = true;
        for (x, y) in curr_pos.iter() {
            if *y as isize - 1 < 0 || grid.contains(&(*x, *y-1)) {
                // if grid contains (x+1, y) then there is already a rock in that position and it cannot move.
                can_move_down = false;
                break;
            }
        }
        can_move_down
    }

    fn move_rock_down(&self, curr_pos: &mut Vec<(usize, usize)>, grid: &HashSet<(usize, usize)>) {
        todo!()
    }

    fn place_rock(&self, curr_pos: &Vec<(usize, usize)>, grid: &mut HashSet<(usize, usize)>) {
        todo!()
    }

}
/*
    Plus appears at (2, n) where n is current height
*/
pub fn solve_day_17() {
    let mut grid: HashSet<(usize, usize)> = HashSet::new(); // Will contain the points of all rocks
    let mut rock_count = 0;
    let mut gases_iter = INPUT.chars().map(|c| Gas::new(c) ).into_iter().cycle();
    let rocks = vec![Rocks::HorizontalLine, Rocks::Plus, Rocks::LShape, Rocks::VerticalLine, Rocks::Square];
    let mut rocks_iter = rocks.iter().cycle();
    while rock_count < 10 {
        let current_rock = rocks_iter.next().unwrap();
        let height = get_height_of_rocks(&grid) + 3 ; // Plus three since height will start 3 above the highest rock
        let mut curr_pos = current_rock.get_starting_positions(height);
        'falling: loop {
            // Starting dropping rocks
            let gas = gases_iter.next().unwrap();
            match gas {
                Gas::Right => current_rock.move_rock_right(&mut curr_pos, &grid),
                Gas::Left => current_rock.move_rock_left(&mut curr_pos, &grid),
            }

            // Check below rock before stop
            if !current_rock.can_rock_move_down(&mut curr_pos, &grid) {
                break 'falling;
            }

            // Rock can keep falling
            current_rock.move_rock_down(&mut curr_pos, &grid);
        }
        // Place rock
        current_rock.place_rock(&curr_pos, &mut grid);
    }
    
    // println!("Height: {}", get_height_of_rocks(&grid));
    println!("Height of grid: {}", grid.len());
}

fn get_height_of_rocks(grid: &HashSet<(usize, usize)>) -> usize {
    let mut max_y = 0;
    grid.iter().for_each(|(_, y)| {
        if *y > max_y {
            max_y = *y
        }
    });
    max_y
}
