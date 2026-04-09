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
    frutis: Option<String>,
    csvfile: Option<String>,
}

fn csv_to_vec(csv: &str) -> Vec<String> {
    csv.split(',')
        .map(|s| s.trim().to_string())
        .collect()
}

fn display_fruit_salad(fruits: Vec<String>) {
    println!("Your fruit salad contains: ");
    for fruit in fruits {
        print!("{}", fruit);
    }
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