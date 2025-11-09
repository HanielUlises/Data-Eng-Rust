mod cli;
mod ingest;
mod graph;

use crate::{cli::Args, ingest::*, graph::*};
use anyhow::Result;
use clap::Parser;
use rayon::prelude::*;
use serde::Serialize;
use std::fs::File;
use std::io::BufWriter;

#[derive(Serialize)]
struct CentralityResult {
    name: String,
    closeness: f32,
    betweenness: f32,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let records = load_fighters(&args.input)?;
    println!("Loaded {} fighters from {}", records.len(), args.input);

    let (graph, nodes) = build_fight_graph(&records);

    let (closeness, betweenness): (Vec<_>, Vec<_>) = rayon::join(
        || closeness_centrality(&graph, &nodes),
        || betweenness_centrality(&graph, &nodes),
    );

    let mut results: Vec<_> = closeness
        .into_iter()
        .map(|(name, cl)| CentralityResult {
            name: name.clone(),
            closeness: cl,
            betweenness: 0.0,
        })
        .collect();

    for (name, bet) in betweenness {
        if let Some(r) = results.iter_mut().find(|r| r.name == name) {
            r.betweenness = bet;
        }
    }

    let out_file = File::create(&args.output)?;
    let writer = BufWriter::new(out_file);
    serde_json::to_writer_pretty(writer, &results)?;
    println!("Wrote centrality results to {}", args.output);

    println!("\n=== Centrality Summary ===");
    for r in &results {
        println!(
            "{:20}  closeness: {:.4}  betweenness: {:.4}",
            r.name, r.closeness, r.betweenness
        );
    }

    Ok(())
}