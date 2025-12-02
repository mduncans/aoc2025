use clap::ValueEnum;

#[derive(Debug, PartialEq, Eq, Clone, ValueEnum)]
pub enum Part {
    One,
    Two,
}
