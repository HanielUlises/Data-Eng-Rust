use csv::StringRecord;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Deserialize)]
pub struct FighterRecord {
    pub name: String,
    pub wins: u32,
    pub losses: u32,
}

pub fn load_fighters<P: AsRef<Path>>(path: P) -> Result<Vec<FighterRecord>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: FighterRecord = result?;
        records.push(record);
    }
    Ok(records)
}

pub fn build_fight_graph(
    records: &[FighterRecord],
) -> (petgraph::graph::UnGraph<&FighterRecord, f32>, Vec<petgraph::graph::NodeIndex>) {
    use petgraph::graph::{NodeIndex, UnGraph};

    let mut graph = UnGraph::<&FighterRecord, f32>::new_undirected();
    let mut node_map: HashMap<&str, NodeIndex> = HashMap::new();

    for rec in records {
        let idx = graph.add_node(rec);
        node_map.insert(&rec.name, idx);
    }

    let names = [
        "Dustin Poirier",
        "Khabib Nurmagomedov",
        "Jose Aldo",
        "Conor McGregor",
        "Nate Diaz",
    ];
    let idx = |name: &str| *node_map.get(name).expect("name must exist");

    let edges = [
        (0, 1, 1.0), // Poirier ↔ Khabib
        (1, 3, 1.0), // Khabib   ↔ McGregor
        (3, 0, 1.0), // McGregor ↔ Poirier
        (3, 2, 1.0), // McGregor ↔ Aldo
        (3, 4, 1.0), // McGregor ↔ Diaz
        (0, 4, 1.0), // Poirier ↔ Diaz
        (2, 4, 1.0), // Aldo    ↔ Diaz
    ];

    for (a, b, w) in edges {
        let a = names[a];
        let b = names[b];
        graph.add_edge(idx(a), idx(b), w);
    }
    // -------------------------------------------------

    let nodes: Vec<NodeIndex> = node_map.values().copied().collect();
    (graph, nodes)
}