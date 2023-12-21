use std::collections::{HashMap, VecDeque};

use bitvec::BitVec64;
use itertools::Itertools;
use mygrid::{grid::Grid, point::Point};
use regex::Regex;

advent_of_code::solution!(20);

#[derive(Clone, Hash)]
enum Gate {
    FlipFlop(bool),
    Conjunction(BitVec64, u32),
    PassThrough,
}

struct Graph {
    edges: Grid<bool>, // true in (1, 3) means 1 -> 3
    node_data: Vec<Gate>,
    name_to_idx: HashMap<String, usize>,
}

impl Graph {
    fn new() -> Self {
        Self {
            edges: Grid::new(100, 100, false),
            node_data: Vec::with_capacity(100),
            name_to_idx: HashMap::new(),
        }
    }

    fn get_or_add_node(&mut self, name: &str, gate: Gate) -> usize {
        if let Some(idx) = self.name_to_idx.get(name) {
            match self.node_data[*idx] {
                Gate::PassThrough => {
                    self.node_data[*idx] = gate;
                }
                _ => {}
            }
            *idx
        } else {
            let idx = self.node_data.len();
            self.node_data.push(gate);
            self.name_to_idx.insert(name.to_owned(), idx);
            idx
        }
    }

    fn connect(&mut self, from: &str, to: &str) {
        let from_idx = self.get_or_add_node(from, Gate::PassThrough);
        let to_idx = self.get_or_add_node(to, Gate::PassThrough);
        self.edges[Point::new_usize(from_idx, to_idx)] = true;
    }

    // returns the number idx of the button
    fn parse(&mut self, input: &str) -> usize {
        // node idx of the button
        let line_re = Regex::new(r"(?:(?<gate_type>%|&)(?<gate_name>\w+)|(?<broadcaster>broadcaster)) -> (?<output_gates>(?:\w|,| )+)").unwrap();

        self.get_or_add_node("button", Gate::PassThrough);
        self.get_or_add_node("broadcaster", Gate::PassThrough);
        self.connect("button", "broadcaster");

        for line in input.lines().filter(|l| !l.is_empty()) {
            let caps = line_re.captures(line).unwrap();
            let output_gates = caps
                .name("output_gates")
                .unwrap()
                .as_str()
                .split(", ")
                .collect_vec();

            let name = if let Some(_) = caps.name("broadcaster") {
                self.get_or_add_node("broadcaster", Gate::PassThrough);
                "broadcaster"
            } else {
                let gate_type = caps.name("gate_type").unwrap().as_str();
                let gate_name = caps.name("gate_name").unwrap().as_str();
                let gate = match gate_type {
                    "%" => Gate::FlipFlop(false),
                    "&" => Gate::Conjunction(BitVec64::from_size(32), 0),
                    _ => panic!("Unknown gate type {}", gate_type),
                };
                self.get_or_add_node(gate_name, gate);
                gate_name
            };

            for output_name in output_gates {
                self.connect(name, output_name);
            }
        }

        let count = self.node_data.len();
        self.edges
            .resize_to_max_point(Point::new_usize(count, count), false);

        // set conjunction inboud count
        for (idx, gate) in self.node_data.iter_mut().enumerate() {
            if let Gate::Conjunction(_, count) = gate {
                let input_count = self
                    .edges
                    .row(idx)
                    .iter()
                    .filter(|&&is_connected| is_connected)
                    .count();
                *count = input_count as u32;
            }
        }

        0
    }

    #[inline]
    fn get_output_gate_indices(&self, gate_idx: usize) -> Vec<usize> {
        self.edges
            .row(gate_idx)
            .iter()
            .enumerate()
            .filter_map(|(idx, &is_connected)| if is_connected { Some(idx) } else { None })
            .collect_vec()
    }

    #[inline]
    fn apply_signal(
        &mut self,
        gate_idx: usize,
        input_idx: usize,
        input_pulse: bool,
    ) -> Option<bool> {
        let gate = &self.node_data[gate_idx];
        let (gate, output_signal) = match gate {
            Gate::FlipFlop(is_on) => match (is_on, input_pulse) {
                (_, true) => (Gate::FlipFlop(*is_on), None),
                (false, false) => (Gate::FlipFlop(true), Some(true)),
                (true, false) => (Gate::FlipFlop(false), Some(false)),
            },
            Gate::Conjunction(input_memory, input_count) => {
                let mut new_memory = input_memory.clone();
                new_memory.set(input_idx, input_pulse);
                let has_all_high = new_memory.count_ones() == *input_count;
                if has_all_high {
                    (Gate::Conjunction(new_memory, *input_count), Some(false))
                } else {
                    (Gate::Conjunction(new_memory, *input_count), Some(true))
                }
            }
            Gate::PassThrough => (Gate::PassThrough, Some(input_pulse)),
        };
        self.node_data[gate_idx] = gate;

        output_signal
    }

    #[inline]
    fn get_name(&self, gate_idx: usize) -> &str {
        self.name_to_idx
            .iter()
            .find(|(_, &idx)| idx == gate_idx)
            .unwrap()
            .0
    }

    #[inline]
    fn debug_print_graph(&self) {
        println!("================= Graph =================");
        for (name, idx) in &self.name_to_idx {
            let gate_type = match &self.node_data[*idx] {
                Gate::FlipFlop(_) => "%",
                Gate::Conjunction(_, _) => "&",
                Gate::PassThrough => "=",
            };
            println!(
                "{}{} -> {}",
                gate_type,
                name,
                self.get_output_gate_indices(*idx)
                    .iter()
                    .map(|&idx| self.get_name(idx))
                    .join(", ")
            );
        }
        println!("=================== = ===================");
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut graph = Graph::new();
    graph.parse(input);

    graph.debug_print_graph();
    let mut total_low = 0;
    let mut total_high = 0;

    let mut graph = graph;
    for i in 0..1000 {
        //println!("================= Cycle {} BEGIN =================", i);
        let mut q = VecDeque::with_capacity(1000);
        q.push_back((0, 0, false));

        while let Some((gate_idx, input_idx, input_pulse)) = q.pop_front() {
            let output_pulse = graph.apply_signal(gate_idx, input_idx, input_pulse);

            for output_idx in graph.get_output_gate_indices(gate_idx) {
                if let Some(output_pulse) = output_pulse {
                    if output_pulse {
                        total_high += 1;
                    } else {
                        total_low += 1;
                    }
                    println!(
                        "{} -{}-> {}",
                        graph.get_name(gate_idx),
                        output_pulse,
                        graph.get_name(output_idx)
                    );
                    q.push_back((output_idx, gate_idx, output_pulse));
                }
            }
        }
        //println!("================= Cycle {} END =================", i);
    }

    Some(total_high * total_low)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, 2));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(32000000));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, None);
    }
}
