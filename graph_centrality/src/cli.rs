use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(short, long, default_value = "fighters.csv")]
    pub input: String,

    #[arg(short, long, default_value = "centrality.json")]
    pub output: String,
}