use anyhow::Result as AnyhowResult;
use std::path::Path;

pub fn parse_instructions(path: impl AsRef<Path>) -> AnyhowResult<()> {
    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn can_parse_input() {
    }
}
