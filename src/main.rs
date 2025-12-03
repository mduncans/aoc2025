use anyhow::{Result as AnyhowResult, bail};
use clap::Parser;
use utils::Part;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Path to input file
    input: String,

    /// which day to use
    #[arg(short, long, value_name = "DAY")]
    day: i32,

    /// which part to solve 1 or 2
    #[arg(short, long, default_value = "one", value_name = "PART")]
    part: Part,
}

fn try_main() -> AnyhowResult<()> {
    let args = Args::parse();
    let result = match args.day {
        1 => day_one::day_one(&args.input, args.part)?,
        2 => day_two::day_two(&args.input, args.part)?,
        _ => bail!("Please use a valid day"),
    };
    println!(
        "The answer to day {}, input {}, is: \n\t {result}",
        args.day, args.input
    );
    Ok(())
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{e:?}");
        std::process::exit(1)
    }
}
