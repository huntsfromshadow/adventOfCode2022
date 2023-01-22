use std::collections::HashMap;
use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Valve {
    flow_rate: i32,
    edges: Vec<String>,
}
impl Valve {
    fn build(flow_rate: i32) -> Valve {
        Valve { flow_rate: flow_rate, edges: Vec::new() }
    }

    fn add_edge(&mut self, to_id: String) {
        self.edges.push(to_id.clone());
    }
}

#[derive(Debug)]
struct Graph {
    edge_map: HashMap<String, Valve>,
}
impl Graph {
    fn build() -> Graph {
        Graph {
            edge_map: HashMap::new()
        }
    }

    fn set_valve_flow(&mut self, id: String, flow: i32) {
        self.edge_map.entry(id).and_modify(|v| {
            v.flow_rate = flow;
        }).or_insert(Valve::build(flow));
    }

    fn add_edge(&mut self, from_id: String, to_id: String) {
        self.edge_map.entry(to_id.clone()).and_modify(|v| {
            v.add_edge(to_id);
        }).or_insert(Valve::build(0));
    }
}

fn load_graph(fname: &str) -> Graph {
    let mut retval = Graph::build();

    let d1 = fs::read_to_string(fname);
    let d2 = d1.unwrap();
    let dat = d2.lines();

    let re = Regex::new(r"Valve (.*) has flow rate=(.*); tunnel[s]? lead[s]? to valve[s]? (.*)")
        .unwrap();
    
    dat.for_each(|l| {
        let r = re.captures(l).unwrap();
        let id = String::from(&r[1]);
        let flow = String::from(&r[2]).parse::<i32>().unwrap();
        let e1 = String::from(&r[3]);
        let edges = e1.split(",")
            .map(|l| {
                return String::from(l).trim().to_string();
            });

        retval.set_valve_flow(id.clone(), flow);
        edges.for_each(|ed| {
            retval.add_edge(id.clone(), ed);
        });
    });
    retval
}

fn main() {

    let g = load_graph("/Users/huntsfromshadow/code/adventOfCode2022/day16/rustversion/short_input.txt");
    dbg!(g);

}
