mod instructions;

use anyhow::Result as AnyhowResult;
use utils::Part;
use std::path::Path;

use crate::instructions::parse_instructions;

fn solve() -> AnyhowResult<()> {
    Ok(())
}

pub fn day_two(input: impl AsRef<Path>, part: Part) -> AnyhowResult<i32> {
    let _instructions = parse_instructions(input)?;
    let _ = solve();

    let result = match part {
        Part::One => 1,
        Part::Two => 2,
    };

    Ok(result)
}
