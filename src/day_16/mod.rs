use std::collections::{VecDeque, HashMap};

const TEST_INPUT: &str = include_str!("input_day_16.txt");

#[derive(Debug, Clone)]
struct Tunnel {
    pub valve_name: String,
    pub rate: i32,
    pub adj_tunnels: Vec<String>,
    pub is_opened: bool,
    pub parent: Option<String>,
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
                parent: None,
            })
        })
        .collect::<HashMap<String, Tunnel>>()
}

pub fn solve_day_16() {
    let mut tunnels = parse_input(TEST_INPUT);
    solve_part_one(&mut tunnels, "AA".to_string());
}

fn solve_part_one(tunnels: &mut HashMap<String, Tunnel>, starting: String) {
    println!("final res: {}", bfs(tunnels, starting, 0, 30));
}

fn bfs(tunnels: &mut HashMap<String, Tunnel>, starting: String, existing_calc: usize, time_remaining: usize) -> usize {
    // Base case, run out of time
    if time_remaining <= 0 {
        return existing_calc
    }
    // Set up for bfs
    let mut queue: VecDeque<String> = VecDeque::new();
    let mut explored: Vec<String> = vec![starting.clone()];
    queue.push_back(tunnels.get(&starting).unwrap().valve_name.clone());

    let mut distance: HashMap<String, usize> = HashMap::new(); // Hashmap of all distances with respect to starting point
    distance.insert(starting.to_string(), 0);

    let mut calculations = Vec::new();
    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();
        let v_dist = distance.get(&v).unwrap().clone();

        let adj = tunnels.get(&v).unwrap().adj_tunnels.clone();
        for w in adj {
            if !explored.contains(&w) {
                // Get the distance
                let d = distance.entry(w.clone()).and_modify(|d| *d = v_dist + 1).or_insert(v_dist+1);
                let rate = tunnels.get(&w).unwrap().rate as isize;
                // Calculate the Time * Rate
                let calc = ((time_remaining as isize - *d as isize - 1) as isize) * rate;
                let t = tunnels.get(&w).unwrap();
                if calc > 0 && !t.is_opened {
                    calculations.push((w.clone(), calc as usize , time_remaining-*d-1));
                }
                explored.push(w.clone());
                queue.push_back(w);
            }
        }
    }
    calculations.sort_by(|a, b| a.1.cmp(&b.1));
    if calculations.len() == 0 {
        return existing_calc
    }

    let res = calculations.iter().map(|d| {
        let mut tunnels = tunnels.clone();
        tunnels.entry(d.0.to_string()).and_modify(|t| t.is_opened = true);
        bfs(&mut tunnels, d.0.clone(), d.1, d.2)
    }).max().unwrap();
    res + existing_calc
    
}

/*
    Algorithm idea:
        Start at t = 30 and AA, visit all nodes in graph
        while visiting node, if node.rate == 0, just decrease t
        if node.rate > 0, new_t = decrease t by 2, then multiply new_t * rate
        After visiting all nodes, pick the max node that maximizes Time * Rate
        Set Node to open
        Start at t = new_t


*/