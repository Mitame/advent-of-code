use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader, Read},
};

use crate::Aoc;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum GateType {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Gate {
    gtype: GateType,
    input_a: String,
    input_b: String,
    output: String,
}

impl Gate {
    fn get_output_and_update_state(
        &self,
        state: &mut HashMap<String, bool>,
        gates: &HashMap<String, Gate>,
        max_depth: usize,
    ) -> Option<bool> {
        if max_depth == 0 {
            return None;
        }
        if let Some(output) = state.get(&self.output) {
            return Some(*output);
        }

        let a = state.get(&self.input_a).copied().or_else(|| {
            let gate = gates.get(&self.input_a).unwrap();
            gate.get_output_and_update_state(state, gates, max_depth - 1)
        });
        let b = state.get(&self.input_b).copied().or_else(|| {
            let gate = gates.get(&self.input_b).unwrap();
            gate.get_output_and_update_state(state, gates, max_depth - 1)
        });

        let result = a.zip(b).map(|(a, b)| match self.gtype {
            GateType::And => a && b,
            GateType::Or => a || b,
            GateType::Xor => a ^ b,
        });

        if let Some(result) = result {
            state.insert(self.output.clone(), result);
        }

        result
    }

    fn depends_on<'a>(&'a self, gates: &'a HashMap<String, Gate>) -> HashSet<&'a String> {
        let a_gate = gates.get(&self.input_a);
        let b_gate = gates.get(&self.input_b);

        let a_depends = a_gate
            .map(|gate| gate.depends_on(gates))
            .unwrap_or_default();
        let b_depends = b_gate
            .map(|gate| gate.depends_on(gates))
            .unwrap_or_default();
        a_depends
            .union(&b_depends)
            .copied()
            .chain([&self.input_a, &self.input_b, &self.output])
            .collect()
    }
}

fn reverse_depends<'a>(
    on: &'a String,
    gates: &'a HashMap<String, Gate>,
    depth: usize,
) -> HashSet<&'a String> {
    if depth == 0 {
        return HashSet::new();
    }
    gates
        .values()
        .filter(|gate| &gate.input_a == on || &gate.input_b == on)
        .flat_map(|g| reverse_depends(&g.output, gates, depth - 1))
        .chain([on])
        .collect()
}

struct Data {
    state: HashMap<String, bool>,
    gates: Vec<Gate>,
}

fn parse(buf: &mut dyn Read) -> Data {
    let reader = BufReader::new(buf);
    let lines: Result<Vec<_>, _> = reader.lines().collect();
    let lines = lines.unwrap();
    let split_point = lines
        .iter()
        .enumerate()
        .find_map(|(i, v)| v.is_empty().then_some(i))
        .unwrap();
    let (state_lines, gate_lines) = lines.split_at(split_point);

    let state = state_lines
        .iter()
        .map_while(|line| {
            if line.is_empty() {
                return None;
            }

            let (input, state) = line.split_once(':')?;
            match state.trim() {
                "0" => Some((input.to_string(), false)),
                "1" => Some((input.to_string(), true)),
                _ => panic!("unrecognised state"),
            }
        })
        .collect();

    let gates = gate_lines
        .iter()
        .skip(1)
        .map_while(|line| {
            let (gate, output) = line.split_once("->")?;

            let (input_a, remaining) = gate.trim().split_once(' ')?;
            let (gate_type, input_b) = remaining.trim().split_once(' ')?;

            let gate_type = match gate_type.trim() {
                "AND" => GateType::And,
                "OR" => GateType::Or,
                "XOR" => GateType::Xor,
                _ => panic!("Invalid gate type"),
            };

            Some(Gate {
                input_a: input_a.trim().to_string(),
                input_b: input_b.trim().to_string(),
                gtype: gate_type,
                output: output.trim().to_string(),
            })
        })
        .collect();

    Data { state, gates }
}

fn part1(buf: &mut dyn Read) {
    let Data { gates, mut state } = parse(buf);
    let gates: HashMap<_, _> = gates.into_iter().map(|g| (g.output.clone(), g)).collect();

    let mut z_keys: Vec<_> = gates
        .keys()
        .chain(state.keys())
        .filter(|k| k.starts_with('z'))
        .collect();
    z_keys.sort();
    let z_gates: Vec<_> = z_keys.into_iter().map(|k| gates.get(k).unwrap()).collect();
    let z_values: Vec<_> = z_gates
        .into_iter()
        .map(|g| {
            g.get_output_and_update_state(&mut state, &gates, 1000)
                .unwrap()
        })
        .collect();
    let mut result: usize = 0;
    for (i, _) in z_values.into_iter().enumerate().filter(|(_, v)| *v) {
        result |= 1 << i;
    }
    println!("Part 1: {}", result);
}

fn get_initial_state(x: usize, y: usize, bits: u8) -> HashMap<String, bool> {
    let mut state: HashMap<String, bool> = HashMap::new();
    for i in 0..bits {
        state.insert(format!("x{:02}", i), x & (1 << i) != 0);
        state.insert(format!("y{:02}", i), y & (1 << i) != 0);
    }
    state
}

fn calculate_network_result(
    x: usize,
    y: usize,
    bits: u8,
    gates: &HashMap<String, Gate>,
) -> Option<usize> {
    let mut state = get_initial_state(x, y, bits);

    let mut z_keys: Vec<_> = gates
        .keys()
        .chain(state.keys())
        .filter(|k| k.starts_with('z'))
        .collect();
    z_keys.sort();
    let z_gates: Vec<_> = z_keys.into_iter().map(|k| gates.get(k).unwrap()).collect();
    let z_values: Option<Vec<_>> = z_gates
        .into_iter()
        .map(|g| g.get_output_and_update_state(&mut state, gates, 1000))
        .collect();
    let z_values = z_values?;
    let mut result: usize = 0;
    for (i, _) in z_values.into_iter().enumerate().filter(|(_, v)| *v) {
        result |= 1 << i;
    }
    Some(result)
}

fn swap_gates(a: &String, b: &String, gates: &mut HashMap<String, Gate>) {
    let mut a = gates.remove(a).unwrap();
    let b = gates.get_mut(b).unwrap();

    std::mem::swap(&mut a.output, &mut b.output);
    std::mem::swap(&mut a, b);
    gates.insert(a.output.clone(), a);
}

fn check_gates(gates: &HashMap<String, Gate>) -> Option<u8> {
    for i in 0..BITS {
        for x in RANDOM_NUMBERS {
            for y in RANDOM_NUMBERS {
                let z = calculate_network_result(*x, *y, BITS, gates).unwrap_or(0);
                let expected = x + y;

                if z & (1 << i) != expected & (1 << i) {
                    return Some(i);
                }
            }
        }
    }

    None
}

const BITS: u8 = 45;

fn part2(buf: &mut dyn Read) {
    let Data { gates, .. } = parse(buf);
    let mut gates: HashMap<_, _> = gates.into_iter().map(|g| (g.output.clone(), g)).collect();

    let mut swaps = vec![];

    while let Some(incorrect_bit) = check_gates(&gates) {
        let new_gates = gates
            .get(&format!("z{:02}", incorrect_bit))
            .unwrap()
            .depends_on(&gates);
        let prev_gates = if incorrect_bit >= 1 {
            gates
                .get(&format!("z{:02}", incorrect_bit - 1))
                .unwrap()
                .depends_on(&gates)
        } else {
            HashSet::new()
        };

        let x_input = format!("x{:02}", incorrect_bit);
        let y_input = format!("y{:02}", incorrect_bit);

        let possibly_swapped: HashSet<_> = new_gates
            .difference(&prev_gates)
            .copied()
            .chain(reverse_depends(&x_input, &gates, 4))
            .chain(reverse_depends(&y_input, &gates, 4))
            .filter(|g| !g.starts_with('x') && !g.starts_with('y'))
            .collect();
        eprintln!("Something has been swapped from {:?}", possibly_swapped);

        let possibly_swapped: Vec<_> = possibly_swapped.into_iter().collect();
        // Perform the swapping game
        'swapper: for (ai, a) in possibly_swapped.iter().copied().enumerate() {
            for b in possibly_swapped[(ai + 1)..].iter().copied() {
                let mut swapped_gates = gates.clone();
                swap_gates(a, b, &mut swapped_gates);

                let result = check_gates(&swapped_gates);
                if result.is_none() {
                    swaps.push(a.clone());
                    swaps.push(b.clone());
                    eprintln!("Swap solution is {} <> {}", a, b);
                    gates = swapped_gates;
                    break 'swapper;
                } else if let Some(next_incorrect_bit) = result {
                    if next_incorrect_bit > incorrect_bit {
                        swaps.push(a.clone());
                        swaps.push(b.clone());
                        eprintln!("Swap solution is probably {} <> {}", a, b);
                        gates = swapped_gates;
                        break 'swapper;
                    }
                }
            }
        }
    }

    swaps.sort();
    println!("Part 2: {}", swaps.join(","));
}

inventory::submit!(Aoc::new(2024, 24, part1, part2,));

// Harolding
const RANDOM_NUMBERS: &[usize] = &[
    15427882490032,
    9751129520899,
    12834776053818,
    5355392176067,
    11050582836843,
    2444822486653,
    1204863789329,
    6464561016969,
    9423207509795,
    1265498399090,
];
