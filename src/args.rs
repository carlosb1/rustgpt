use clap::Parser;

/// Yet another command-line chat GPT frontend written in Rust.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value_t = false)]
    pub history: bool,
    #[arg(long, default_value_t = false)]
    pub context: bool,
}
