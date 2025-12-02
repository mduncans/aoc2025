use anyhow::Result as AnyhowResult;
use std::error::Error;
use std::str::FromStr;
use std::path::Path;
use std::fmt;
use fs_err as fs;
use utils::Part;



#[derive(Debug, PartialEq, Eq)]
struct ParseDirectionError;

impl fmt::Display for ParseDirectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to parse direction")
    }
}

impl Error for ParseDirectionError {}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ParseDirectionError;
     
    fn from_str(s: &str) -> Result<Self, Self::Err> {
       match s.to_lowercase().as_str() {
            "l" => Ok(Direction::Left),
            "r" => Ok(Direction::Right),
            _ => Err(ParseDirectionError),
       }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseInstructionError;

impl fmt::Display for ParseInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to parse instruction")
    }
}

impl Error for ParseInstructionError {}

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    direction: Direction,
    value: i32,
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir_str, val_str) = s.split_at(1);
        
        let direction = Direction::from_str(dir_str).map_err(|_| ParseInstructionError)?;
        let value= val_str.parse::<i32>().map_err(|_| ParseInstructionError)?;

        Ok(Instruction {
            direction,
            value,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Dial {
    position: i32,
    final_zero_count: i32,
    any_zero_count: i32,
}

impl Default for Dial {
    fn default() -> Self {
        Dial {
            position: 50,
            final_zero_count: 0,
            any_zero_count: 0
        }
    }
}

impl Dial {
    fn perform_instruction(&mut self, direction: Direction, value: i32) {
        let needed_ticks = self.ticks_to_zero(&direction);

        self.position = match direction {
            Direction::Left => {
                (self.position - value).rem_euclid(100)
            },
            Direction::Right => {
                (self.position + value).rem_euclid(100)
            },
        };
        
        if self.position == 0 { self.final_zero_count += 1 };
        if value >= needed_ticks { self.any_zero_count += 1 + (value - needed_ticks) / 100 }
    }

    fn ticks_to_zero(&self, direction: &Direction) -> i32 {
        let ticks = match direction {
            Direction::Left => self.position.rem_euclid(100),
            Direction::Right => (100 - self.position).rem_euclid(100),
        };

        if ticks == 0 { 100 } else { ticks }
    }
}

pub fn day_one(input: impl AsRef<Path>, part: Part) -> AnyhowResult<i32> {
    let path = input.as_ref();

    let mut dial = Dial::default();
    let contents = fs::read_to_string(path)?;
    let instructions= contents.lines()
        .map(Instruction::from_str)
        .collect::<Vec<_>>()
        .into_iter()
        .collect::<Result<Vec<Instruction>, ParseInstructionError>>()?;
    
    instructions
        .into_iter()
        .for_each(|i| dial.perform_instruction(i.direction, i.value));
    
    match part {
        Part::One => Ok(dial.final_zero_count),
        Part::Two => Ok(dial.any_zero_count),
    }
} 

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{Direction, Instruction, Dial};
    #[test]
    fn can_parse_instructions() {
        let instructions = vec![
            ("L68", Instruction { direction: Direction::Left, value: 68}),
            ("L30", Instruction { direction: Direction::Left, value: 30}),
            ("R48", Instruction { direction: Direction::Right, value: 48}),
            ("L5", Instruction { direction: Direction::Left, value: 5}),
            ("R60", Instruction { direction: Direction::Right, value: 60}),
            ("L55", Instruction { direction: Direction::Left, value: 55}),
            ("L1", Instruction { direction: Direction::Left, value: 1}),
            ("L99", Instruction { direction: Direction::Left, value: 99}),
            ("R14", Instruction { direction: Direction::Right, value: 14}),
            ("L82", Instruction { direction: Direction::Left, value: 82}),
        ];

        for (inst_str, answer) in instructions {
            let int = Instruction::from_str(inst_str).unwrap();
            assert_eq!(int, answer);
        }
    }

    #[test]
    fn single_instructions_work() {
        // Starting number, instruction, ending number
        let scenarios = vec![
            (11, "R8", 19),
            (19, "L19", 0),
            (0, "L1", 99),
            (99, "R1", 0),
            (5, "L10", 95),
            (95, "R5", 0),
        ];

        for scenario in scenarios {
            let (start, inst_str, end) = scenario;
            let mut dial = Dial { position: start, final_zero_count: 0, any_zero_count: 0 };
            let instruction = Instruction::from_str(inst_str).unwrap();

            dial.perform_instruction(instruction.direction, instruction.value);
            
            assert_eq!(dial.position, end);
        }
    }

    #[test]
    fn can_count_final_zeros() {
        let instructions = vec![
            "L68",
            "L30",
            "R48",
            "L5",
            "R60",
            "L55",
            "L1",
            "L99",
            "R14",
            "L82",
        ];
        
        let mut dial = Dial::default();

        for inst_str  in instructions {
            let int = Instruction::from_str(inst_str).unwrap();
            dial.perform_instruction(int.direction, int.value);
        }

        assert_eq!(dial.final_zero_count, 3);
    }

    #[test]
    fn can_count_any_zero() {
        let instructions = vec![
            "L68",
            "L30",
            "R48",
            "L5",
            "R60",
            "L55",
            "L1",
            "L99",
            "R14",
            "L82",
        ];

        let mut dial = Dial::default();

        for inst_str  in instructions {
            let int = Instruction::from_str(inst_str).unwrap();
            dial.perform_instruction(int.direction, int.value);
        }

        assert_eq!(dial.any_zero_count, 6);
    }
}
