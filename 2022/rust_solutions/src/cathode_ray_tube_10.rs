use std::{error, fmt};

#[derive(Debug)]
enum Instruction {
    Addx,
    Noop,
}

#[derive(Debug)]
pub struct ProgramLine {
    inst: Instruction,
    param: Option<isize>,
}

#[derive(Debug, Clone)]
struct ProgramLineParseError;

impl fmt::Display for ProgramLineParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid test to parse to ProgramLine or Instruction")
    }
}

impl error::Error for ProgramLineParseError {}

impl ProgramLine {
    pub fn parse(program_line_str: &str) -> Result<ProgramLine, Box<dyn error::Error>>
    {
        let line_vector = program_line_str.split(" ")
            .collect::<Vec<&str>>();
        let program_line = match line_vector[0] {
            "addx" => ProgramLine {
                inst: Instruction::Addx,
                param: Some(line_vector[1].parse::<isize>().unwrap()),
            },
            "noop" => ProgramLine {
                inst: Instruction::Noop,
                param: None,
            },
            _ => return Err(ProgramLineParseError.into()),
        };


        return Ok(program_line);
    }
}

pub(crate) fn parse_commands(lines: Vec<String>) -> Result<Vec<ProgramLine>, Box<dyn error::Error>> {
    lines.iter()
        .map(|line| ProgramLine::parse(line))
        .collect::<Result<Vec<ProgramLine>, Box<dyn error::Error>>>()
        .map_err(|e| e.into())
}

pub(crate) fn get_signal_strengths_sum(program_lines: Vec<ProgramLine>) -> isize {
    let mut register_state_during_cycle: Vec<isize> = Vec::new();
    let interesting_cycles = vec![20, 60, 100, 140, 180, 220];
    let mut cycle_idx = 0;
    let mut x = 0;
    // initialization
    register_state_during_cycle.push(x);
    for program_line in program_lines.iter() {
        match program_line.inst {
            Instruction::Noop => {
                register_state_during_cycle.push(x);
                cycle_idx += 1;
            }
            Instruction::Addx => {
                register_state_during_cycle.push(x);
                register_state_during_cycle.push(x);
                cycle_idx += 2;
                x += program_line.param.unwrap();
            }
        }
    }
    let mut outputs = register_state_during_cycle
        .iter()
        .enumerate()
        .collect::<Vec<(usize, &isize)>>();
    for output in outputs {
        let (cycle, x) = output;
        println!("Cycle {:?}, X: {:?}", cycle, x)
    }
    return register_state_during_cycle.iter()
        .enumerate()
        .filter(|(index, _)| interesting_cycles.contains(index))
        .map(|(idx, &elem)| idx as isize * elem)
        .collect::<Vec<isize>>()
        .iter()
        .sum();
}