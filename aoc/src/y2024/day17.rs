use std::io::{BufRead, BufReader, Read};

use crate::Aoc;

#[derive(Debug, Clone, PartialEq)]
struct Registers {
    a: usize,
    b: usize,
    c: usize,
    instruction_pointer: usize,
}

#[derive(Debug, Clone)]
enum Operand {
    Literal(u8),
    RegisterA,
    RegisterB,
    RegisterC,
}

impl Operand {
    fn parse_combo(operand: u8) -> Self {
        match operand {
            0..=3 => Self::Literal(operand),
            4 => Self::RegisterA,
            5 => Self::RegisterB,
            6 => Self::RegisterC,
            _ => panic!("Invalid Combo Operand: {}", operand),
        }
    }

    fn literal(operand: u8) -> Self {
        Operand::Literal(operand)
    }

    fn value(&self, registers: &Registers) -> usize {
        match self {
            Self::Literal(value) => *value as usize,
            Self::RegisterA => registers.a,
            Self::RegisterB => registers.b,
            Self::RegisterC => registers.c,
        }
    }
}

#[derive(Debug, Clone)]
enum Operator {
    Divide(Operand),
    BitwiseXorLiteral(Operand),
    StoreB(Operand),
    JumpNotZero(Operand),
    BitwiseXorC,
    Output(Operand),
    DivideB(Operand),
    DivideC(Operand),
}

impl Operator {
    fn parse(instruction: u8, operand: u8) -> Result<Self, ()> {
        match instruction {
            0 => Ok(Self::Divide(Operand::parse_combo(operand))),
            1 => Ok(Self::BitwiseXorLiteral(Operand::literal(operand))),
            2 => Ok(Self::StoreB(Operand::parse_combo(operand))),
            3 => Ok(Self::JumpNotZero(Operand::literal(operand))),
            4 => Ok(Self::BitwiseXorC),
            5 => Ok(Self::Output(Operand::parse_combo(operand))),
            6 => Ok(Self::DivideB(Operand::parse_combo(operand))),
            7 => Ok(Self::DivideC(Operand::parse_combo(operand))),
            _ => Err(()),
        }
    }

    fn perform(&self, registers: &mut Registers) -> Option<u8> {
        match self {
            Self::Divide(operand) => {
                let result = Self::perform_division(registers, operand);
                registers.a = result;
                None
            }
            Operator::BitwiseXorLiteral(operand) => {
                registers.b ^= operand.value(registers);
                None
            }
            Operator::StoreB(operand) => {
                registers.b = operand.value(registers) & 0b111;
                None
            }
            Operator::JumpNotZero(operand) => {
                if registers.a != 0 {
                    registers.instruction_pointer = operand.value(registers);
                }
                None
            }
            Operator::BitwiseXorC => {
                registers.b ^= registers.c;
                None
            }
            Operator::Output(operand) => Some((operand.value(registers) & 0x7) as u8),
            Operator::DivideB(operand) => {
                let result = Self::perform_division(registers, operand);
                registers.b = result;
                None
            }
            Operator::DivideC(operand) => {
                let result = Self::perform_division(registers, operand);
                registers.c = result;
                None
            }
        }
    }

    fn perform_division(registers: &Registers, operand: &Operand) -> usize {
        let numerator = registers.a;
        let denominator_power = operand.value(registers);
        numerator.overflowing_shr((denominator_power) as u32).0
    }
}

struct Data {
    registers: Registers,
    instructions: Vec<u8>,
}

fn parse(buf: &mut dyn Read) -> Data {
    let reader = BufReader::new(buf);

    let mut registers = Registers {
        a: 0,
        b: 0,
        c: 0,
        instruction_pointer: 0,
    };
    let mut instructions: Vec<u8> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let Some((label, data)) = line.split_once(':') else {
            continue;
        };
        let data = data.trim();
        match label {
            "Register A" => {
                registers.a = data.parse().unwrap();
            }
            "Register B" => {
                registers.b = data.parse().unwrap();
            }
            "Register C" => {
                registers.c = data.parse().unwrap();
            }
            "Program" => instructions.extend(data.split(',').map(|v| v.parse::<u8>().unwrap())),
            _ => {
                panic!("Bleh")
            }
        }
    }

    Data {
        registers,
        instructions,
    }
}

fn run_step(registers: &mut Registers, instructions: &[u8]) -> Result<Option<u8>, ()> {
    let instruction = instructions[registers.instruction_pointer];
    let operand = instructions[registers.instruction_pointer + 1];
    registers.instruction_pointer += 2;

    let operator = Operator::parse(instruction, operand);
    Ok(operator?.perform(registers))
}

fn run_program(registers: &mut Registers, instructions: &[u8]) -> Result<Vec<u8>, ()> {
    let max_instruction_pointer = instructions.len();
    let mut outputs = Vec::new();
    while registers.instruction_pointer < max_instruction_pointer {
        let output = run_step(registers, instructions)?;
        if let Some(output) = output {
            outputs.push(output);
        }
    }
    Ok(outputs)
}

fn part1(buf: &mut dyn Read) {
    let Data {
        mut registers,
        instructions,
    } = parse(buf);

    let outputs = run_program(&mut registers, &instructions).unwrap();

    let results: Vec<String> = outputs.iter().map(|v| v.to_string()).collect();
    let result = results.join(",");
    println!("Part 1: {}", result);
}

fn find_number(
    registers: &Registers,
    instructions: &[u8],
    position: usize,
    start_value: usize,
) -> Option<usize> {
    for i in 0..8 {
        let value = start_value + i;
        let output = run_program(
            &mut Registers {
                a: value,
                ..registers.clone()
            },
            instructions,
        )
        .unwrap();

        if output.len() > position
            && output[output.len() - position - 1]
                == instructions[instructions.len() - position - 1]
        {
            if (position + 1) == instructions.len() {
                return Some(value);
            } else {
                let Some(result) = find_number(registers, instructions, position + 1, value * 8)
                else {
                    continue;
                };
                return Some(result);
            }
        }
    }
    None
}

fn part2(buf: &mut dyn Read) {
    let Data {
        registers,
        instructions,
    } = parse(buf);

    let result = find_number(&registers, &instructions, 0, 0).unwrap();
    println!("Part 2: {}", result);
}

inventory::submit!(Aoc::new(
    2024,
    17,
    part1,
    part2,
    include_bytes!("./inputs/day17")
));

#[cfg(test)]
mod tests {
    use super::*;

    fn run_and_return_registers(
        registers: &mut Registers,
        instructions: &[u8],
    ) -> (Vec<u8>, Registers) {
        let output = run_program(registers, instructions).unwrap();
        (output, registers.clone())
    }

    #[test]
    fn test_run_program() {
        assert_eq!(
            run_and_return_registers(
                &mut Registers {
                    a: 0,
                    b: 0,
                    c: 9,
                    instruction_pointer: 0,
                },
                &[2, 6]
            ),
            (
                vec![],
                Registers {
                    a: 0,
                    b: 1,
                    c: 9,
                    instruction_pointer: 2,
                }
            )
        );

        assert_eq!(
            run_and_return_registers(
                &mut Registers {
                    a: 10,
                    b: 0,
                    c: 0,
                    instruction_pointer: 0,
                },
                &[5, 0, 5, 1, 5, 4]
            ),
            (
                vec![0, 1, 2],
                Registers {
                    a: 10,
                    b: 0,
                    c: 0,
                    instruction_pointer: 6,
                }
            )
        );

        assert_eq!(
            run_and_return_registers(
                &mut Registers {
                    a: 2024,
                    b: 0,
                    c: 0,
                    instruction_pointer: 0,
                },
                &[0, 1, 5, 4, 3, 0]
            ),
            (
                vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0],
                Registers {
                    a: 0,
                    b: 0,
                    c: 0,
                    instruction_pointer: 6,
                }
            )
        );

        assert_eq!(
            run_and_return_registers(
                &mut Registers {
                    a: 0,
                    b: 29,
                    c: 0,
                    instruction_pointer: 0,
                },
                &[1, 7]
            ),
            (
                vec![],
                Registers {
                    a: 0,
                    b: 26,
                    c: 0,
                    instruction_pointer: 2,
                }
            )
        );

        assert_eq!(
            run_and_return_registers(
                &mut Registers {
                    a: 0,
                    b: 2024,
                    c: 43690,
                    instruction_pointer: 0,
                },
                &[4, 0]
            ),
            (
                vec![],
                Registers {
                    a: 0,
                    b: 44354,
                    c: 43690,
                    instruction_pointer: 2,
                }
            )
        );
    }
}
