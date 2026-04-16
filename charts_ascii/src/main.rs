extern crate rasciigraph;
use rasciigraph::{plot, Config};

fn main() {
    let cities = vec!["Lisbon"," Madrid"," Paris"," Berlin"," Copenhaguen"," Stockholm"," Moscow"];
    let distance_travelled = vec![0.0, 502.56, 1053.36, 2187.27, 2636.42, 3117.23, 4606.35];

    println!("{}", cities.join(" > "));

    print!("{}", plot(distance_travelled.into_iter().map(|d| d as f64).collect(), Config::default().with_caption("Travelled Distances (km)".to_string()).with_offset(10).with_height(10)))
}
