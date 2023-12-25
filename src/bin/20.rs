use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(20);

#[derive(Debug, Clone, PartialEq, Copy)]
enum Power {
    High,
    Low,
}

#[derive(Debug, Clone)]
struct Signal {
    from: String,
    power: Power,
    to: String,
}

trait Gate {
    fn apply_signal(&mut self, signal: &Signal) -> Vec<Signal>;
    fn get_downtream_gates(&self) -> Vec<String>;
    fn register_inputs(&mut self, inputs: Vec<String>);
}

#[derive(Debug, Clone)]
struct FlipFlop {
    is_on: bool,
    connections: Vec<String>,
}

impl Gate for FlipFlop {
    fn apply_signal(&mut self, signal: &Signal) -> Vec<Signal> {
        match signal.power {
            Power::High => vec![],
            Power::Low => {
                self.is_on = !self.is_on;
                self.connections
                    .iter()
                    .map(|to| Signal {
                        from: signal.to.clone(),
                        power: match self.is_on {
                            true => Power::High,
                            false => Power::Low,
                        },
                        to: to.clone(),
                    })
                    .collect_vec()
            }
        }
    }
    fn get_downtream_gates(&self) -> Vec<String> {
        self.connections.clone()
    }
    fn register_inputs(&mut self, _inputs: Vec<String>) {}
}

impl FlipFlop {
    fn new(connections: Vec<String>) -> Self {
        Self {
            is_on: false,
            connections,
        }
    }
}

#[derive(Debug, Clone)]
struct Broadcaster {
    connections: Vec<String>,
}

impl Gate for Broadcaster {
    fn apply_signal(&mut self, signal: &Signal) -> Vec<Signal> {
        self.connections
            .iter()
            .map(|to| Signal {
                from: signal.to.clone(),
                power: signal.power.clone(),
                to: to.clone(),
            })
            .collect_vec()
    }
    fn get_downtream_gates(&self) -> Vec<String> {
        self.connections.clone()
    }
    fn register_inputs(&mut self, _inputs: Vec<String>) {}
}

impl Broadcaster {
    fn new(connections: Vec<String>) -> Self {
        Self { connections }
    }
}

#[derive(Debug, Clone)]
struct Conjunction {
    last_input_state: HashMap<String, Power>,
    connections: Vec<String>,
}

impl Gate for Conjunction {
    fn apply_signal(&mut self, signal: &Signal) -> Vec<Signal> {
        self.last_input_state
            .get_mut(signal.from.as_str())
            .map(|input| {
                *input = signal.power;
            });
        let has_all_high = self
            .last_input_state
            .values()
            .all(|&power| power == Power::High);
        self.connections
            .iter()
            .map(|to| Signal {
                from: signal.to.clone(),
                power: match has_all_high {
                    true => Power::Low,
                    false => Power::High,
                },
                to: to.clone(),
            })
            .collect_vec()
    }
    fn get_downtream_gates(&self) -> Vec<String> {
        self.connections.clone()
    }

    fn register_inputs(&mut self, inputs: Vec<String>) {
        self.last_input_state = HashMap::new();
        for input in inputs {
            self.last_input_state.insert(input, Power::Low);
        }
    }
}

impl Conjunction {
    fn new(connections: Vec<String>) -> Self {
        Self {
            last_input_state: HashMap::new(),
            connections,
        }
    }
}

struct System {
    gates: HashMap<String, Box<dyn Gate>>,
}

impl System {
    fn parse(input: &str) -> Self {
        let line_re =
            Regex::new(r"(?:(?<gate_type>%|&|b)(?<gate_name>\w+)) -> (?<output_gates>(?:\w|,| )+)")
                .unwrap();

        let mut gates: HashMap<String, Box<dyn Gate>> = HashMap::new();
        let mut input_to_register: HashMap<String, Vec<String>> = HashMap::new();
        for line in input.lines().filter(|l| !l.is_empty()) {
            let caps = line_re.captures(line).unwrap();
            let output_gates = caps
                .name("output_gates")
                .unwrap()
                .as_str()
                .split(", ")
                .map(|s| s.to_string())
                .collect_vec();

            let gate_name = caps.name("gate_name").unwrap().as_str().to_string();
            let gate_type = caps.name("gate_type").unwrap().as_str();
            match gate_type {
                "%" => {
                    let gate = Box::new(FlipFlop::new(output_gates.clone()));
                    gates.insert(gate_name.clone(), gate);
                }
                "&" => {
                    let gate = Box::new(Conjunction::new(output_gates.clone()));
                    gates.insert(gate_name.clone(), gate);
                }
                "b" => {
                    let gate = Box::new(Broadcaster::new(output_gates.clone()));
                    gates.insert("broadcaster".to_string(), gate);
                }
                _ => panic!("Unknown gate type {}", gate_type),
            };

            for output_gate in output_gates {
                let mut inputs = match input_to_register.get(&output_gate) {
                    Some(inputs) => inputs.clone(),
                    None => Vec::new(),
                };
                inputs.push(gate_name.clone());
                input_to_register.insert(output_gate, inputs);
            }
        }

        // set inputs for all gates
        for (name, gate) in gates.iter_mut() {
            let inputs = match input_to_register.get(name) {
                Some(inputs) => inputs.clone(),
                None => Vec::new(),
            };
            gate.register_inputs(inputs);
        }

        // register target gates that do not exists in the input left side
        let mut undeclared_gates: Vec<String> = Vec::new();
        for (_, gate) in gates.iter() {
            let downstream_gates = gate.get_downtream_gates();
            for downstream_gate in downstream_gates {
                if !gates.contains_key(&downstream_gate) {
                    undeclared_gates.push(downstream_gate);
                }
            }
        }
        for undeclared_gate in undeclared_gates {
            let gate = Box::new(Broadcaster::new(vec![]));
            gates.insert(undeclared_gate, gate);
        }

        Self { gates }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut system = System::parse(input);
    let mut total_low = 0;
    let mut total_high = 0;

    for _ in 0..1000 {
        //println!("================= Cycle {} BEGIN =================", i);
        let mut q = VecDeque::with_capacity(100);
        q.push_back(Signal {
            from: "button".to_string(),
            power: Power::Low,
            to: "broadcaster".to_string(),
        });

        while let Some(signal) = q.pop_front() {
            match signal.power {
                Power::High => total_high += 1,
                Power::Low => total_low += 1,
            }
            let gate = system.gates.get_mut(&signal.to).unwrap();
            let new_signals = gate.apply_signal(&signal);
            q.extend(new_signals);
        }
        //println!("================= Cycle {} END =================", i);
    }

    Some(total_high * total_low)
}

pub fn part_two(input: &str) -> Option<u64> {
    let loop_end_nodes = vec!["bp", "xc", "th", "pd"];
    let btn_press_needed = loop_end_nodes
        .iter()
        .map(|&must_be_high| {
            // do button presses until we see high on the required node
            let mut system = System::parse(input);
            let mut total_btn_presses = 0;
            loop {
                let mut q = VecDeque::with_capacity(100);
                q.push_back(Signal {
                    from: "button".to_string(),
                    power: Power::Low,
                    to: "broadcaster".to_string(),
                });
                total_btn_presses += 1;

                while let Some(signal) = q.pop_front() {
                    if signal.from == *must_be_high && signal.power == Power::High {
                        return total_btn_presses;
                    }
                    let gate = system.gates.get_mut(&signal.to).unwrap();
                    let new_signals = gate.apply_signal(&signal);
                    q.extend(new_signals);
                }
            }
        })
        .collect_vec();

    println!("btn_press_needed: {:?}", btn_press_needed.clone());

    // pgcm between all nodes
    let mut lcm: u64 = 1;
    for btn_press in btn_press_needed {
        lcm = num::integer::lcm(lcm, btn_press);
    }
    Some(lcm)
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
