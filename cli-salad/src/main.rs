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
    fruits: Option<String>,
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
        print!("{} ", fruit);
    }
}

fn main() {
    let opts: Opts = Opts::parse();

    let fruit_list = match opts.csvfile {
        Some(filename ) => {
            let fruits = std::fs::read_to_string(filename)
                .expect("Could not read the file");
            csv_to_vec(&fruits)
        },
        None => {
            opts.fruits.unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect()
        },
    };

    let fruit_salad = create_fruit_salad(fruit_list);
    display_fruit_salad(fruit_salad);
}