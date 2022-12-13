
/*
    Packets come in pairs
    Within a packet, either a number or another [] can be found
    Top packet in pair is Left, Other is Right
    
*/

use std::cmp::Ordering;

const INPUT: &str = include_str!("input_day_13.txt");

#[derive(Debug, Clone)]
struct  Packet {
    items: String
}

impl Packet {
    pub fn new(line: &str) -> Packet {
        Packet { items: line.to_string() }
    }
}

impl PartialEq for Packet {
    /**
     * For a packet to be equal to other packet, they must contain the same items
     */
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
    }
}

pub fn handle_vectors(left: &str, right: &str) -> Option<Ordering> {
    if left.is_empty() && !right.is_empty() {
        return Some(Ordering::Less);
    }
    if !left.is_empty() && right.is_empty() {
        return Some(Ordering::Greater);
    }
    if left.is_empty() && right.is_empty() {
        return None
    }
    println!("left: {:?} and right: {:?}", left, right);
    let left = left.split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let right = right.split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    println!("Left u32 vec: {:?}, right u32 vec: {:?}", left, right);
    for i in 0..left.len() {
        if right.len()-1 < i {
            return None;
        }
        match left[i].cmp(&right[i]) {
            Ordering::Less => return Some(Ordering::Less),
            Ordering::Equal => continue,
            Ordering::Greater => return Some(Ordering::Greater),
        }
    }
    // All elements are equal, check to see if left is less than right
    println!("left and right have equal elements: ording: {:?}", left.len().partial_cmp(&right.len()));
    return left.len().partial_cmp(&right.len());
}

fn get_matching_end_bracket(current_index: usize, str: &str) -> usize{
    let mut open_brace_count = 0;
    let mut matching_end_bracket = current_index+1;
    for end_bracket in str[current_index+1..].chars() {
        matching_end_bracket += 1;
        if end_bracket != '[' && end_bracket != ']' {
            continue;
        }
        if end_bracket == '[' {
            open_brace_count += 1;
            continue;
        }
        if open_brace_count != 0 {
            open_brace_count -= 1;
            continue;
        }
        break;
    };
    return matching_end_bracket
}

fn handle_mixed_types(current_val: &str, current_str: &str, index: usize, existing_packet: Packet) -> Option<Ordering> {
    let current_val = if current_val == "1" && index + 2 < current_str.len() && current_str[index..index+2].chars().nth(index+1).is_some() && current_str[index..index+2].chars().nth(index+1).unwrap().is_digit(10) {
        format!("{}{}", current_val, current_str[index..index+2].chars().nth(index+1).unwrap())
    } else { current_val.to_string() };
    let current_str_packet = Packet::new(format!("[{}]", current_val).as_str());
    return existing_packet.partial_cmp(&current_str_packet)
}

impl PartialOrd for Packet {
    /**
     * Comparing packets
     */
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let left = &self.items[1..self.items.len()-1];
        let right = &other.items[1..other.items.len()-1];
        // Handles inputs with no sublists
        if !left.contains("[") && !right.contains("[") {
            return handle_vectors(left, right)
        }

        let mut left_iter = left.char_indices();
        let mut right_iter = right.char_indices();
        loop {
            let (_i, _left_ch) = match left_iter.next() {
                None => {
                    if right_iter.count() > 0 {
                        return Some(Ordering::Less)
                    } else {
                        return Some(Ordering::Equal)
                    }
                },
                Some(left) => left
            };
            let mut i = _i;
            let mut left_ch = _left_ch;
            let (_j, _right_ch) = match right_iter.next() {
                None => return Some(Ordering::Greater),
                Some(right) => right
            };
            let mut j = _j;
            let mut right_ch = _right_ch;
            while left_ch == ']' || left_ch == ',' {
                (i, left_ch) = match left_iter.next() {
                    None => {
                        if right_iter.count() > 0 {
                            return Some(Ordering::Less)
                        } else {
                            return Some(Ordering::Equal)
                        }
                    },
                    Some(left) => left
                }
            }
            while right_ch == ']' || right_ch == ',' {
                (j, right_ch) = match right_iter.next() {
                    None => return Some(Ordering::Greater),
                    Some(right) => right
                }
            }
            // println!("left ch: {} and right ch: {}", left_ch, right_ch);
            match left_ch {
                '[' => {
                    let matching_end_bracket = get_matching_end_bracket(i, left);
                    let inner_packet = Packet::new(&left[i..matching_end_bracket]);
                    // println!("Inner packet: {:?}", inner_packet);
                    // Check to see if right is packet or number
                    match right_ch {
                        '[' => {
                            // finding matching end
                            let matching_end_bracket = get_matching_end_bracket(j, right);
                            // println!("Before right inner packet");
                            let right_inner_packet = Packet::new(&right[j..matching_end_bracket]);
                            // println!("right inner packet: {:?}", right_inner_packet);
                            return match inner_packet.partial_cmp(&right_inner_packet) {
                                None => None,
                                Some(res) => {
                                    if res == Ordering::Equal {
                                        continue;
                                    }
                                    return Some(res);
                                }
                            }
                        },
                        ch => {
                            return match handle_mixed_types(
                                ch.to_string().as_str(),
                                right,
                                j,
                                inner_packet
                            ) {
                                None => None,
                                Some(res) => {
                                    if res == Ordering::Equal {
                                        continue;
                                    }
                                    return Some(res);
                                }
                            }
                        }
                    }
                },
                ch => {
                    let ch = if ch == '1' && i + 2 < left.len() && left[i..i+2].chars().nth(i+1).is_some() && left[i..i+2].chars().nth(i+1).unwrap().is_digit(10) {
                        format!("{}{}", ch, left[i..i+2].chars().nth(i+1).unwrap())
                    } else { ch.to_string() };

                    match right_ch {
                        '[' => {
                            // Mixed types
                            // finding matching end
                            let matching_end_bracket = get_matching_end_bracket(j, right);
                            let right_inner_packet = Packet::new(&right[j..matching_end_bracket]);
                           
                            let current_str_packet = Packet::new(format!("[{}]", ch).as_str());
                            return current_str_packet.partial_cmp(&right_inner_packet)
                            
                        },
                        val => {
                            // right char is number, and left char is number, compare it
                            let val = if val == '1' && right[j..j+2].chars().nth(j+1).is_some() && right[j..j+2].chars().nth(j+1).unwrap().is_digit(10) {
                                format!("{}{}", val, right[j..j+2].chars().nth(j+1).unwrap())
                            } else { val.to_string() };
                            return match ch.to_string().parse::<u32>().unwrap().partial_cmp(&val.parse::<u32>().unwrap()) {
                                None => None,
                                Some(res) => {
                                    if res == Ordering::Equal {
                                        continue;
                                    }
                                    return Some(res);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}


pub fn solve_day_13() {
    let solution = INPUT
        .split("\r\n\r\n")
        .enumerate()
        .filter_map(|(i, packets)| {
            let packets = packets.split_whitespace().collect::<Vec<&str>>();
            // println!("Left: {} right: {}", packets[0], packets[1]);
            let left = Packet::new(packets[0]);
            let right = Packet::new(packets[1]);
            if left.partial_cmp(&right) == Some(Ordering::Less) {
                return Some(i+1)
            }
            None
        }).sum::<usize>();
        // .collect::<Vec<usize>>();
    println!("solution: {:?}", solution);
}

#[test]
fn test_packet_equality() {
    let left = Packet::new("[1,1,3,1,1]");
    let right = Packet::new("[1,1,3,1,1]");
    assert!(left == right);
}

#[test]
fn test_packet_left_is_less_than() {
    let left = Packet::new("[1,1,3,1,1]");
    let right = Packet::new("[1,1,5,1,1]");
    assert!(left < right);
}

#[test]
fn test_packet_left_is_less_than2() {
    let left = Packet::new("[1,[2,[3,[4,[5,6,7]]]],8,9]");
    let right = Packet::new("[1,[2,[3,[4,[5,6,0]]]],8,9]");
    assert_eq!(left < right, false);
}

#[test]
fn test_packet_left_is_less_than3() {
    let left = Packet::new("[[1],[2,3,4]]");
    let right = Packet::new("[[1],4]");
    assert_eq!(left.partial_cmp(&right), Some(Ordering::Less));
}

#[test]
fn test_packet_4() {
    let left = Packet::new("[[[]]]");
    let right = Packet::new("[[]]");
    assert_eq!(left < right, false);
}

#[test]
fn test_packet_5() {
    let left = Packet::new("[[4,4],4,4]");
    let right = Packet::new("[[4,4],4,4,4]");
    assert_eq!(left.partial_cmp(&right), Some(Ordering::Less));
}