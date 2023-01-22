// Thank you to https://raw.githubusercontent.com/WinterCore/aoc2022/main/day16/main.

use std::{fs, str::FromStr, collections::HashMap, usize};

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    rate: u32,
    connections: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseValveError;

impl FromStr for Valve {
    type Err = ParseValveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .trim()
            .split_once(";")
            .ok_or(ParseValveError)?;

        let (raw_name, raw_rate) = a
            .trim()
            .split_once(" has flow ")
            .ok_or(ParseValveError)?;

        let name = raw_name
            .trim()
            .strip_prefix("Valve ")
            .and_then(|s| Some(String::from(s)))
            .ok_or(ParseValveError)?;

        let rate = raw_rate
            .trim()
            .strip_prefix("rate=")
            .and_then(|s| s.parse::<u32>().ok())
            .ok_or(ParseValveError)?;

        let connections = Some(b.trim())
            .and_then(|s| s.strip_prefix("tunnel ").or(s.strip_prefix("tunnels ")))
            .and_then(|s| s.strip_prefix("lead ").or(s.strip_prefix("leads ")))
            .and_then(|x| x.strip_prefix("to "))
            .and_then(|s| s.strip_prefix("valve ").or(s.strip_prefix("valves ")))
            .and_then(|s| Some(
                s.split(",")
                .map(|c| c.trim().to_string())
                .collect::<Vec<String>>()
            )).ok_or(ParseValveError)?;

        Ok(Valve { name, rate, connections })
    }
}

#[derive(Debug)]
struct SimpleValve {
    name: String,
    rate: u32,
    links: Vec<usize>,
}

fn parse(input: &str) -> Vec<SimpleValve> {
    let valves: Vec<Valve> = input.lines()
        .filter(|l| ! l.trim().is_empty())
        .map(|l| l.parse::<Valve>().unwrap())
        .collect();
    
    let idx_map: HashMap<String, usize> = valves.iter()
        .enumerate()
        .fold(HashMap::new(), |mut m, (i, x)| {
            m.insert(x.name.clone(), i);
            m
        });


    valves.into_iter()
        .map(|v| SimpleValve {
            name: v.name,
            rate: v.rate,
            links: v.connections.iter().map(|x| *idx_map.get(x).unwrap()).collect(),
        })
        .collect()
}

/*
 * Convert the valves into a weighted graph and generate a min distance matrix by using floyd warshall
 *
 *
 * How we keep track of open valves
 * Let's say that we have a list of 8 valves, and the starting valve is at index 3
 * The starting bitmask in this case would look like this
 * 0b11110111
 *
 * Open valves are represented by 0s and closed ones are 1s
 *
 * the size of the bitmask is based on how many valves we want to keep track of
 * in our case the size is 2^8 - 1
 * 2^8     = 0b100000000
 * 2^8 - 1 = 0b011111111
 *
 * As perform the simulation we modify the bitmask according to what valves are opened, and we
 * keep track of the highest possible flow rate out of each path combination
 * We also use a hashmap that keeps track the highest flow rate that we can get out of
 * different valve combinations (This will be important for part2)
 *
 * # Part 1
 *
 * This part is pretty straightforward, we can just use the highest flow rate returned by the
 * simulation 
 * 
 *
 * # Part 2
 *
 * Here, we simulate the paths of the elf and the elpehant separately which results in 2 path
 * hashmaps 
 *
 * After that we loop over each of the elf's paths and find paths that the elephant took
 * that don't overlap (using some bitmasks magic) with the elf path being checked;
 * Overlap here means that both of them opened the same valves, so we basically discard all
 * paths where both of them have at least 1 valve in common because a valve can only be opened
 * once.
 *
 * (! elf_mask) & (! elephant_mask) & init_mask == 0
 * The reason why we flip elf_mask's and elephant_mask's bits is because open valves are represented by 0s
 * instead of 1s (not sure why I thought this was a good idea) and then we have to AND the result
 * with the initial mask because when we flip the bits of a number, all of the bits are flipped (depending on the integer size being used)
 * and not only the ones being used, for example if we have 0b0011 that is stored within a u16 and
 * we flip the bits we'll end up with 0b1111111111111100, as you can see the flipping will happen
 * across all 16 bits, and that's why we AND the result with the initial_mask.
 *
 * If the result of the above bitmask expression results in a number that's bigger than 0 that
 * means that we have overlaps and we have to skip the current pair.
 * For pairs that don't have overlaps we add their highest flow rates together and compare them
 * with the rest.
 */
fn main() {
    let contents = fs::read_to_string("./input")
        .expect("File not found");

    let parsed = parse(&contents);


    println!("Part 1: {}", part1(&parsed)); // ~15ms
    println!("Part 2: {}", part2(&parsed)); // ~50ms
}

fn part1(valves: &Vec<SimpleValve>) -> String {
    let graph = init_graph(valves, |x| &x.links);
    let dist = floyd_warshall(graph);

    let start_idx = valves.iter().position(|x| x.name == "AA").unwrap();
    let len = dist.len();

    // I spent 20 hours trying to find a bug that later appeared to be happening because of this line
    // APPARENTLY THE STARTING VALVE IS ALWAYS "AA" AND NOT THE FIRST VALVE IN THE INPUT
    let init_mask: u64 = (1 << len) - 1;

    let (flow, _) = simulate(valves, &dist, init_mask, start_idx, 30);

    String::from(flow.to_string())
}

fn part2(valves: &Vec<SimpleValve>) -> String {
    let graph = init_graph(valves, |x| &x.links);
    let dist = floyd_warshall(graph);

    let start_idx = valves.iter().position(|x| x.name == "AA").unwrap();
    let init_mask: u64 = (1 << dist.len()) - 1;

    let (_, elf_memo) = simulate(valves, &dist, init_mask, start_idx, 26);
    let (_, elephant_memo) = simulate(valves, &dist, init_mask, start_idx, 26);

    let max_flow = elf_memo
        .iter()
        .fold(0, |max, (&elf_mask, &elf_flow)| {
            elephant_memo.iter()
                .fold(max, |max, (&mask, &elephant_flow)| {
                    // Check that there's no overlap between the 2 paths
                    if (! mask) & (! elf_mask) & init_mask == 0 {
                        return max.max(elephant_flow + elf_flow);
                    }

                    max
                })
        });


    String::from(max_flow.to_string())
}

fn simulate(
    valves: &Vec<SimpleValve>,
    dist: &Vec<Vec<u32>>,
    init_mask: u64,
    start_idx: usize,
    minutes: u32,
) -> (u32, HashMap<u64, u32>) {
    let non_zero_valves: Vec<usize> = valves
        .iter()
        .enumerate()
        .filter(|(_, x)| x.rate > 0)
        .map(|(i, _)| i)
        .collect();

    let flow = 0;
    let mut mask_flow: HashMap<u64, u32> = HashMap::new();

    let flow = traveling_salesman(valves, &mut mask_flow, &non_zero_valves, &dist, init_mask, minutes, flow, start_idx, 0);

    (flow, mask_flow)
}

fn traveling_salesman(
    valves: &Vec<SimpleValve>,
    memo: &mut HashMap<u64, u32>,
    non_zero_valves: &Vec<usize>,
    dist: &Vec<Vec<u32>>,
    mask: u64,
    minutes: u32,
    flow: u32,
    i: usize,
    depth: u32,
) -> u32 {

    let mut max_flow = flow;

    memo.insert(mask, *memo.get(&mask).unwrap_or(&0).max(&flow));

    for &j in non_zero_valves.iter() {
        let cur_minutes = minutes
            .checked_sub(dist[i][j])
            .and_then(|x| x.checked_sub(1))
            .unwrap_or(0);

        if (mask & (1 << j)) == 0 || cur_minutes <= 0 {
            continue;
        }

        let cur_mask = mask & ! (1 << j);

        let cur_flow = flow + (cur_minutes * valves[j].rate);

        max_flow = max_flow.max(
            traveling_salesman(valves, memo, non_zero_valves, dist, cur_mask, cur_minutes, cur_flow, j, depth + 1)
        );
    }

    return max_flow;
}

fn floyd_warshall(graph: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let l = graph.len();
    let mut dist = graph.clone();

    for k in 0..l {
        for i in 0..l {
            for j in 0..l {
                if dist[i][k] + dist[k][j] < dist[i][j] {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }

    dist
}

fn init_graph<T, F>(list: &Vec<T>, get_links: F) -> Vec<Vec<u32>>
    where F: Fn(&T) -> &Vec<usize> {
    let l = list.len();
    let mut graph = vec![vec![u32::MAX / 4; l]; l];

    list
        .iter()
        .enumerate()
        .for_each(|(i, x)| {
            get_links(x).iter().for_each(|&j| graph[i][j as usize] = 1);
        });

    graph
}

