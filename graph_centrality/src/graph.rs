use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::Direction;
use std::collections::HashMap;

/// Closeness centrality for every node
pub fn closeness_centrality<N, E>(
    graph: &UnGraph<N, E>,
    nodes: &[NodeIndex],
) -> Vec<(String, f32)>
where
    N: Clone + std::fmt::Display,
    E: Copy + Into<f32>,
{
    nodes
        .iter()
        .map(|&node| {
            let dists = dijkstra(graph, node, None, |e| *e.weight());
            let sum: f32 = dists.values().copied().sum();
            let n = graph.node_count() as f32;
            let closeness = if sum > 0.0 { (n - 1.0) / sum } else { 0.0 };
            (graph[node].to_string(), closeness)
        })
        .collect()
}

/// Brandes’ betweenness centrality (normalized)
pub fn betweenness_centrality<N, E>(
    graph: &UnGraph<N, E>,
    nodes: &[NodeIndex],
) -> Vec<(String, f32)>
where
    N: Clone + std::fmt::Display,
    E: Copy,
{
    let n = graph.node_count();
    let mut bet = HashMap::<NodeIndex, f32>::new();

    for &s in nodes {
        let mut stack = Vec::new();
        let mut pred: HashMap<NodeIndex, Vec<NodeIndex>> = HashMap::new();
        let mut sigma: HashMap<NodeIndex, f32> = HashMap::new();
        let mut dist: HashMap<NodeIndex, f32> = HashMap::new();
        let mut queue = std::collections::VecDeque::new();

        for &node in nodes {
            sigma.insert(node, 0.0);
            dist.insert(node, f32::MAX);
            pred.insert(node, Vec::new());
        }

        sigma.insert(s, 1.0);
        dist.insert(s, 0.0);
        queue.push_back(s);

        while let Some(v) = queue.pop_front() {
            stack.push(v);
            for w in graph.neighbors(v) {
                if dist[&w].is_infinite() {
                    queue.push_back(w);
                    dist.insert(w, dist[&v] + 1.0);
                }
                if (dist[&w] - dist[&v]).abs() < 1e-6 {
                    *sigma.get_mut(&w).unwrap() += sigma[&v];
                    pred.get_mut(&w).unwrap().push(v);
                }
            }
        }

        let mut delta: HashMap<NodeIndex, f32> = nodes.iter().map(|&n| (n::std::clone::Clone::clone(&n), 0.0)).collect();

        while let Some(w) = stack.pop() {
            for &v in &pred[&w] {
                let contrib = sigma[&v] / sigma[&w] * (1.0 + delta[&w]);
                *delta.get_mut(&v).unwrap() += contrib;
            }
            if w != s {
                *bet.entry(w).or_insert(0.0) += delta[&w];
            }
        }
    }

    let norm = ((n - 1) * (n - 2)) as f32;
    bet.into_iter()
        .map(|(node, val)| (graph[node].to_string(), val / norm))
        .collect()
}