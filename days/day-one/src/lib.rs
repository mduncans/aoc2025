mod dial;
mod instructions;

use crate::dial::ZeroCounter;
use crate::instructions::{Instruction, parse_instructions};
use anyhow::Result as AnyhowResult;
use std::path::Path;
use utils::Part;

fn solve(instructions: Vec<Instruction>) -> ZeroCounter {
    let mut counter = ZeroCounter::new(50, 100);

    for instruction in instructions {
        counter.execute(instruction);
    }

    counter
}

pub fn day_one(input: impl AsRef<Path>, part: Part) -> AnyhowResult<i32> {
    let instructions = parse_instructions(input)?;
    let counter = solve(instructions);

    let result = match part {
        Part::One => counter.final_zero_count(),
        Part::Two => counter.any_zero_count(),
    };

    Ok(result)
}
