use num::integer::lcm;
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
};

aoc::main!();
aoc::check!("inputs/20.in", 834323022, 225386464601017);

fn part_1(input: &Input) -> u64 {
    input.clear();
    part_(input).0
}

fn part_2(input: &Input) -> u64 {
    input.clear();
    part_(input).1
}

fn part_(input: &Input) -> (u64, u64) {
    let (mut low, mut high) = (1000u64, 0u64);
    let mut pi = 0;
    let mut rsin = HashMap::new();
    while pi < 1_000 || rsin.len() < 4 {
        pi += 1;
        let mut q = VecDeque::new();
        q.push_back((Box::new(String::from(BROADCASTER)), false));
        while let Some((g, pulse)) = q.pop_front() {
            let Some(gate) = input.gates.get(&*g) else {
                continue;
            };
            let output = gate.borrow_mut().tp.recv(pulse);
            if output.is_none() {
                continue;
            }
            let output = output.unwrap();
            if pi <= 1000 {
                if output {
                    high += input.gates[&*g].borrow().outputs.len() as u64;
                } else {
                    low += input.gates[&*g].borrow().outputs.len() as u64;
                }
            }
            for out in &input.gates[&*g].borrow().outputs {
                let other_gate = input.gates.get(out);
                if other_gate.is_none() {
                    continue;
                }
                if out == "rs" && output && !rsin.contains_key(&*g) {
                    rsin.insert(g.clone(), pi);
                }
                q.push_back((Box::new(out.clone()), output));
                let other_gate = other_gate.unwrap();
                if let Gate {
                    tp: GateType::Conjunction { ref mut state },
                    ..
                } = *other_gate.borrow_mut()
                {
                    *state.get_mut(&*g).unwrap() = output;
                }
            }
        }
    }
    (
        low * high,
        rsin.values().fold(1u64, |acc, x| lcm(acc, *x as u64)),
    )
}

struct Input {
    gates: HashMap<String, RefCell<Gate>>,
}
impl Input {
    fn clear(&self) {
        for gate in self.gates.values() {
            gate.borrow_mut().tp.clear();
        }
    }
}
struct Gate {
    tp: GateType,
    outputs: Vec<String>,
}

enum GateType {
    Broadcaster,
    FlipFlop { state: bool },
    Conjunction { state: HashMap<String, bool> },
}
impl GateType {
    fn recv(&mut self, pulse: bool) -> Option<bool> {
        match self {
            GateType::FlipFlop { state } => {
                if pulse {
                    None
                } else {
                    *state = !*state;
                    Some(*state)
                }
            }
            GateType::Conjunction { state } => Some(!state.values().all(|x| *x)),
            GateType::Broadcaster => Some(pulse),
        }
    }
    fn clear(&mut self) {
        match self {
            GateType::FlipFlop { state } => {
                *state = false;
            }
            GateType::Conjunction { state } => {
                for s in state.values_mut() {
                    *s = false;
                }
            }
            GateType::Broadcaster => {}
        }
    }
}
const BROADCASTER: &str = &"broadcaster";

fn load_input(raw: String) -> Input {
    let gates: HashMap<String, RefCell<Gate>> = raw
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("->").unwrap();
            let mut a = a.trim();
            let b = b.trim().split(", ").map(|x| String::from(x)).collect();
            let gate = RefCell::new(if a == BROADCASTER {
                Gate {
                    tp: GateType::Broadcaster,
                    outputs: b,
                }
            } else if a.chars().next().unwrap() == '%' {
                a = &a[1..];
                Gate {
                    tp: GateType::FlipFlop { state: false },
                    outputs: b,
                }
            } else if a.chars().next().unwrap() == '&' {
                a = &a[1..];
                Gate {
                    tp: GateType::Conjunction {
                        state: HashMap::new(),
                    },
                    outputs: b,
                }
            } else {
                panic!();
            });
            (String::from(a), gate)
        })
        .collect();
    for (label, gate) in &gates {
        for out in &gate.borrow().outputs {
            let other_gate = gates.get(out);
            if other_gate.is_none() {
                continue;
            }
            let other_gate = other_gate.unwrap();
            if let Gate {
                tp: GateType::Conjunction { ref mut state },
                ..
            } = *other_gate.borrow_mut()
            {
                state.insert(label.clone(), false);
            }
        }
    }
    Input { gates }
}
