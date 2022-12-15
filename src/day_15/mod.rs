use std::collections::HashSet;

const INPUT: &str = include_str!("input_15.txt");


pub fn solve_day_15() {
    const HORIZONTAL_LINE: isize = 2000000;
    let mut count = 0;
    let mut line: HashSet<(isize, isize)> = HashSet::new();

    INPUT.lines().for_each(|info| {
        let info = info.trim().replace("Sensor at ", ""); 
        let info = info.split(": closest beacon is at ").map(|p| {
            let points = p.split(", ").map(|s| {
                let s = s.replace("x=", "");
                let s = s.replace("y=", "");
                s.clone()
            }).collect::<Vec<String>>();
            (points[0].parse::<isize>().unwrap(), points[1].parse::<isize>().unwrap())
        }).collect::<Vec<(isize, isize)>>();
        let scanner = info[0];
        let beacon = info[1];
        let manhattan_dist = (scanner.0.abs_diff(beacon.0) + scanner.1.abs_diff(beacon.1)) as isize;
        let distance_to_line = scanner.1.abs_diff(HORIZONTAL_LINE) as isize;
        let amount_of_hashtags = (2 * manhattan_dist + 1) - 2 * distance_to_line;
        if 
            scanner.1 <= HORIZONTAL_LINE && (scanner.1 + manhattan_dist) >= HORIZONTAL_LINE ||
            scanner.1 >= HORIZONTAL_LINE && (scanner.1 - manhattan_dist) <= HORIZONTAL_LINE
        {
            count += amount_of_hashtags;
            let count_on_either_side = (amount_of_hashtags - 1)/2;
            // Left
            for i in 0..=count_on_either_side {
                line.insert((scanner.0-i,HORIZONTAL_LINE));
            }
            // right
            for i in 1..=count_on_either_side {
                line.insert((scanner.0+i,HORIZONTAL_LINE));
            }
        }
        line.remove(&beacon);
    });

    println!("count: {}", line.len())
}