
const INPUT: &str = include_str!("test_input.txt");

pub fn solve_day_twelve() {
    let mut grid: Vec<Vec<String>> = vec![];
    for line in INPUT.lines() {
        let mut row: Vec<String> = vec![];
        for char in line.chars() {
            row.push(char.to_string())
        }
        grid.push(row);
    }

    println!("{:#?}", grid);
}