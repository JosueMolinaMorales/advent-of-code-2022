use std::collections::HashMap;


const INPUT: &str = include_str!("test_input.txt");

/**
 *      To calculate the circle around a point, go up, right, down, and left one then between all those points, fill in the area with #
 *      This function assumes the target is 'B'
 */
fn calculate_area_around_point(points: &mut HashMap<(isize, isize), &str>, center: (isize, isize)) {
    let mut beacon_found = false;
    let mut dx = 1;
    let mut dy = 1;
    while !beacon_found {
        // println!("Beacon not found yet for: {:?} currently on dx: {dx}", center);
        let top = (center.0, center.1 - dy);
        let right = (center.0 + dx, center.1);
        let bottom = (center.0, center.1 + dy );
        let left = (center.0 - dx, center.1);

        // Fill in points from top to right
        let mut current_point = top.clone();
        while current_point != right {
            // Go down one and right one
            match points.get(&current_point) {
                None => {},
                Some(point) => beacon_found = *point == "B"
            }
            points.entry(current_point).or_insert("#");
            current_point = (current_point.0 + 1, current_point.1 + 1);
        }

        // Fill in points from right to bottom
        let mut current_point = right.clone();
        while current_point != bottom {
            // Go down one and left one
            match points.get(&current_point) {
                None => {},
                Some(point) => beacon_found = *point == "B"
            }
            points.entry(current_point).or_insert("#");
            current_point = (current_point.0 - 1, current_point.1 + 1);
        }

        // Fill in points from bottom to left
        let mut current_point = bottom.clone();
        while current_point != left {
            // Go down one and left one
            // println!("Bottom to left for: {:?}, current point: {:?}", center, current_point);
            match points.get(&current_point) {
                None => {},
                Some(point) => beacon_found = *point == "B"
            }
            points.entry(current_point).or_insert("#");
            current_point = (current_point.0 - 1, current_point.1 - 1);
        }

        // Fill in points from left to top
        let mut current_point = left.clone();
        while current_point != top {
            // Go down one and left one
            match points.get(&current_point) {
                None => {},
                Some(point) => beacon_found = *point == "B"
            }
            points.entry(current_point).or_insert("#");
            current_point = (current_point.0 + 1, current_point.1 - 1);
        }

        // End of iteration
        dx += 1;
        dy += 1;
    }
}

pub fn solve_day_15() {
    let mut points: HashMap<(isize, isize), &str> = HashMap::new();
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
        points.insert(info[0], "S");
        points.insert(info[1], "B");
    });

    for starting_point in points.clone().iter().filter(|p| *p.1 == "S") {
        println!("Getting area for point: {:?}", starting_point);
        calculate_area_around_point(&mut points, *starting_point.0);
        println!("Finished getting area for: {:?}", starting_point);
        println!("Still contains B's: {}", points.iter().filter(|p| *p.1 == "B").count());
    }
    // println!("{:?}", points);
}

#[test]
fn test_calc_area() {
    let mut points = HashMap::from(
        [
            ((0, 0), "S"),
            ((2, 2), "B")
        ]
    );
    calculate_area_around_point(&mut points, (0, 0));
    let length = points.len();
    println!("length: {}", length);
    assert!(length == 41);
}