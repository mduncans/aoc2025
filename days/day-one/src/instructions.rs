use anyhow::Result as AnyhowResult;
use fs_err as fs;
use std::error::Error;
use std::fmt;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseDirectionError;

impl fmt::Display for ParseDirectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to parse direction")
    }
}

impl Error for ParseDirectionError {}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Direction {
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
pub struct ParseInstructionError;

impl fmt::Display for ParseInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to parse instruction")
    }
}

impl Error for ParseInstructionError {}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Instruction {
    pub direction: Direction,
    pub value: i32,
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir_str, val_str) = s.split_at(1);

        let direction = Direction::from_str(dir_str).map_err(|_| ParseInstructionError)?;
        let value = val_str.parse::<i32>().map_err(|_| ParseInstructionError)?;

        Ok(Instruction { direction, value })
    }
}

pub fn parse_instructions(path: impl AsRef<Path>) -> AnyhowResult<Vec<Instruction>> {
    let contents = fs::read_to_string(path)?;

    let instructions: Result<Vec<Instruction>, _> =
        contents.lines().map(Instruction::from_str).collect();

    let instructions = instructions?;
    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::instructions::{Direction, Instruction};
    #[test]
    fn can_parse_instructions() {
        let instructions = vec![
            (
                "L68",
                Instruction {
                    direction: Direction::Left,
                    value: 68,
                },
            ),
            (
                "L30",
                Instruction {
                    direction: Direction::Left,
                    value: 30,
                },
            ),
            (
                "R48",
                Instruction {
                    direction: Direction::Right,
                    value: 48,
                },
            ),
            (
                "L5",
                Instruction {
                    direction: Direction::Left,
                    value: 5,
                },
            ),
            (
                "R60",
                Instruction {
                    direction: Direction::Right,
                    value: 60,
                },
            ),
            (
                "L55",
                Instruction {
                    direction: Direction::Left,
                    value: 55,
                },
            ),
            (
                "L1",
                Instruction {
                    direction: Direction::Left,
                    value: 1,
                },
            ),
            (
                "L99",
                Instruction {
                    direction: Direction::Left,
                    value: 99,
                },
            ),
            (
                "R14",
                Instruction {
                    direction: Direction::Right,
                    value: 14,
                },
            ),
            (
                "L82",
                Instruction {
                    direction: Direction::Left,
                    value: 82,
                },
            ),
        ];

        for (inst_str, answer) in instructions {
            let int = Instruction::from_str(inst_str).unwrap();
            assert_eq!(int, answer);
        }
    }
}
