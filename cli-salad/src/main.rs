use clap::Parser;
use cli_salad::create_fruit_salad;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Haniel Ulises <hanielulises2003@gmail.com>",
    about = "Number of fruits to include in the salad"
)]

struct Opts{
    #[clap(short, long)]
    number: usize,
    frutis: Option<String>
}

fn main() {
    let opts: Opts = Opts::parse();
    let num_fruits = opts.number;

    create_fruit_salad(num_fruits);

    println!(
        "Created Fruit salad with {} fruits {:?}",
        num_fruits,
        create_fruit_salad(num_fruits)
    );
}