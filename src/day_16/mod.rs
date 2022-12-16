use std::collections::{VecDeque, HashMap};

const TEST_INPUT: &str = include_str!("test_input.txt");

#[derive(Debug)]
struct Tunnel {
    pub valve_name: String,
    pub rate: i32,
    pub adj_tunnels: Vec<String>,
    pub is_opened: bool,
    pub parent: Option<String>
}

fn parse_input(input: &str) -> HashMap<String, Tunnel> {
    input
        .lines()
        .map(|line| {
            let line = line.replace("Valve ", "").replace("has flow rate=", "").replace("; tunnels lead to valves ", " ").replace("; tunnel leads to valve ", " ").replace(", ", ",");
            let line = line.split(" ").collect::<Vec<&str>>();
            // println!("{}", line);
            let tunnel_id = line[0].clone();
            ( tunnel_id.clone().to_string(), Tunnel {
                valve_name: tunnel_id.clone().to_string(),
                rate: line[1].clone().parse::<i32>().unwrap(),
                adj_tunnels: line[2].clone().split(",").map(|s| s.to_string()).collect::<Vec<String>>(),
                is_opened: false,
                parent: None
            })
        })
        .collect::<HashMap<String, Tunnel>>()
}

pub fn solve_day_16() {
    let mut tunnels = parse_input(TEST_INPUT);
    solve_part_one(&mut tunnels, "AA".to_string());
}

fn solve_part_one(tunnels: &mut HashMap<String, Tunnel>, starting: String) {
    let mut queue: VecDeque<String> = VecDeque::new();
    let mut explored: Vec<String> = vec![starting];
    queue.push_back(tunnels.get("AA").unwrap().valve_name.clone());

    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();

        let adj = tunnels.get(&v).unwrap().adj_tunnels.clone();
        for w in adj {
            if !explored.contains(&w) {
                tunnels.entry(w.clone()).and_modify(|t| t.parent = Some(v.clone()));
                explored.push(w.clone());
                queue.push_back(w);
            }
        }
    }
    println!("{:#?}", tunnels);
}


//Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
//AA 0 DD,II,BB