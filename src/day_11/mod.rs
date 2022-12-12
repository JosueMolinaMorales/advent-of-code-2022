use std::collections::HashMap;
use num::{BigUint, FromPrimitive};

const INPUT: &str = include_str!("./input_day_11.txt");

fn get_monkey(lines: &Vec<&str>) -> String {
    lines[0].split(" ").last().unwrap().replace(":", "")
}

fn play_catch(rounds: u32, is_worried: bool) {
    let mut monkeys: HashMap<String, Vec<BigUint>> = HashMap::new();
    let mut monkey_inspection_count: Vec<u32> = vec![];
    for monkey_attr in INPUT.split("\n\n") {
        let lines = monkey_attr.split("\n").collect::<Vec<&str>>();
        let monkey = get_monkey(&lines);
        let starting_levels = lines[1].split(": ").last().unwrap().split(", ").map(|s| BigUint::from_u32(s.parse::<u32>().unwrap()).unwrap()).collect::<Vec<BigUint>>();

        monkey_inspection_count.push(0);
        monkeys.insert(monkey, starting_levels);
    }
    for _ in 1..=rounds {
        for monkey_attr in INPUT.split("\n\n") {
            let lines = monkey_attr.split("\n").collect::<Vec<&str>>();
            let monkey = get_monkey(&lines);
            let operation = lines[2].split(": ").last().unwrap().split(" = ").last().unwrap().split(" ").collect::<Vec<&str>>();
            let divisible_by = lines[3].trim().split(" ").last().unwrap().parse::<u32>().unwrap();
            let if_true_monkey_to = lines[4].trim().split(" ").last().unwrap().to_string();
            let if_false_monkey_to = lines[5].trim().split(" ").last().unwrap().to_string();
            let mut worry_level = monkeys.get_mut(&monkey).unwrap();
            if worry_level.len() == 0 {
                // No items
                continue;
            }

            for _ in 0..worry_level.len() {
                // monkey inspects
                monkey_inspection_count[monkey.parse::<usize>().unwrap()] += 1;
                let old = worry_level[0].clone();
                match operation[1] {
                    "+" => {
                        worry_level[0] += operation[2].parse::<BigUint>().unwrap_or(old);
                    },
                    "*" => {
                        worry_level[0] *= operation[2].parse::<BigUint>().unwrap_or(old);
                    },
                    _ => unreachable!()
                }
                // monkey gets bored, divide by 3
                if is_worried {
                    worry_level[0] /= BigUint::from_i32(3).unwrap();
                }

                // Check divisibility
                if worry_level[0].clone() % divisible_by == BigUint::from_i32(0).unwrap() {
                    // Is divisible
                    let temp_worry = worry_level.remove(0);
                    worry_level = monkeys.get_mut(&if_true_monkey_to).unwrap();
                    worry_level.push(temp_worry);
                } else {
                    let temp_worry = worry_level.remove(0);
                    worry_level = monkeys.get_mut(&if_false_monkey_to).unwrap();
                    worry_level.push(temp_worry);
                }
                // reset work_level back to original monkey
                worry_level = monkeys.get_mut(&monkey).unwrap();
            }
        }
    }
    monkey_inspection_count.sort();
    let res = monkey_inspection_count.iter().rev().take(2).map(|r| *r).collect::<Vec<u32>>();
    println!("Inspection counts: {:?}", res[0] * res[1]);
}

pub fn solve_day_eleven() {
    play_catch(20, true);
    play_catch(10000, false);
}