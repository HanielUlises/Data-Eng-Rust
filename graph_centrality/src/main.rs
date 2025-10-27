use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::algo::dijkstra;
use petgraph::Direction;
use std::fmt;
use std::collections::HashMap;

#[derive(Debug)]
struct Fighter {
    name: String,
}

impl Fighter {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl fmt::Display for Fighter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn add_edge(graph: &mut UnGraph<&Fighter, f32>, nodes: &[NodeIndex], a: usize, b: usize, weight: f32) {
    graph.add_edge(nodes[a], nodes[b], weight);
}

fn calculate_closeness_centrality(graph: &UnGraph<&Fighter, f32>, nodes: &[NodeIndex]) -> Vec<(String, f32)> {
    let mut centralities = Vec::new();
    
    for (i, &node) in nodes.iter().enumerate() {
        let distances = dijkstra(graph, node, None, |e| *e.weight());
        let sum_distances: f32 = distances.values().sum();
        let closeness = if sum_distances > 0.0 {
            (graph.node_count() as f32 - 1.0) / sum_distances
        } else {
            0.0
        };
        centralities.push((graph[node].name.clone(), closeness));
    }
    
    centralities
}

fn calculate_betweenness_centrality(graph: &UnGraph<&Fighter, f32>, nodes: &[NodeIndex]) -> Vec<(String, f32)> {
    let mut betweenness = HashMap::new();
    let n = graph.node_count();
    
    for s in nodes.iter() {
        let mut stack = Vec::new();
        let mut pred: HashMap<NodeIndex, Vec<NodeIndex>> = HashMap::new();
        let mut sigma: HashMap<NodeIndex, f32> = HashMap::new();
        let mut dist: HashMap<NodeIndex, f32> = HashMap::new();
        let mut queue = Vec::new();
        
        for &node in nodes {
            sigma.insert(node, 0.0);
            dist.insert(node, f32::MAX);
            pred.insert(node, Vec::new());
        }
        
        sigma.insert(*s, 1.0);
        dist.insert(*s, 0.0);
        queue.push(*s);
        
        while let Some(v) = queue.pop() {
            stack.push(v);
            for w in graph.neighbors(v) {
                if dist[&w] == f32::MAX {
                    queue.push(w);
                    dist.insert(w, dist[&v] + 1.0);
                }
                if dist[&w] == dist[&v] + 1.0 {
                    *sigma.get_mut(&w).unwrap() += sigma[&v];
                    pred.get_mut(&w).unwrap().push(v);
                }
            }
        }
        
        let mut delta: HashMap<NodeIndex, f32> = HashMap::new();
        for &node in nodes {
            delta.insert(node, 0.0);
        }
        
        while let Some(w) = stack.pop() {
            for &v in pred[&w].iter() {
                *delta.get_mut(&v).unwrap() += sigma[&v] / sigma[&w] * (1.0 + delta[&w]);
            }
            if w != *s {
                *betweenness.entry(w).or_insert(0.0) += delta[&w];
            }
        }
    }
    
    betweenness
        .into_iter()
        .map(|(node, value)| (graph[node].name.clone(), value / ((n - 1) * (n - 2)) as f32))
        .collect()
}

fn main() {
    let mut graph = UnGraph::new_undirected();
    let fighters = [
        Fighter::new("Dustin Poirier"),
        Fighter::new("Khabib Nurmagomedov"),
        Fighter::new("Jose Aldo"),
        Fighter::new("Conor McGregor"),
        Fighter::new("Nate Diaz"),
    ];

    let fighter_nodes: Vec<NodeIndex> = fighters
        .iter()
        .map(|fighter| graph.add_node(fighter))
        .collect();

    add_edge(&mut graph, &fighter_nodes, 0, 1, 1.0);
    add_edge(&mut graph, &fighter_nodes, 1, 3, 1.0);
    add_edge(&mut graph, &fighter_nodes, 3, 0, 1.0);
    add_edge(&mut graph, &fighter_nodes, 3, 2, 1.0);
    add_edge(&mut graph, &fighter_nodes, 3, 4, 1.0);
    add_edge(&mut graph, &fighter_nodes, 0, 4, 1.0);
    add_edge(&mut graph, &fighter_nodes, 2, 4, 1.0);

    println!("Closeness Centrality:");
    let closeness = calculate_closeness_centrality(&graph, &fighter_nodes);
    for (name, centrality) in closeness {
        println!("{}: {:.4}", name, centrality);
    }

    println!("\nBetweenness Centrality:");
    let betweenness = calculate_betweenness_centrality(&graph, &fighter_nodes);
    for (name, centrality) in betweenness {
        println!("{}: {:.4}", name, centrality);
    }
}