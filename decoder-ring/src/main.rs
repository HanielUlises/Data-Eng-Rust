use clap::Parser;
use decoder_ring::print_stats_analysis;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Args {
    #[arg(short, long)]
    message: String,

    #[arg(short, long)]
    stats: bool,

    #[arg(short, long)]
    guess: bool,
}

fn main() {
    let args = Args::parse();
    if args.stats {
        print_stats_analysis(&args.message);
    }

    
}