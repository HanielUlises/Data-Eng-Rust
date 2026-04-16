use textwrap::fill;

struct PageRank{
    damping: f64,
    iterations: usize,
}

impl PageRank {
    fn new(damping: f64, iterations: usize) -> Self {
        Self {damping, iterations}
    }

    fn rank(&self, graph: &Vec<Vec<usize>>) -> Vec<f64> {
        let n = graph.len();
        let mut ranks = vec![1.0/(n as f64); n];

        for _ in 0..self.iterations {
            let mut new_ranks = vec![0.0; n];

            for (node, edges) in graph.iter().enumerate() {
                let contribution = ranks[node] / (edges.len() as f64);

                for &edge in edges{
                    new_ranks[edge] += contribution;
                }
            }

                for rank in &mut new_ranks {
                *rank = *rank * self.damping + (1.0 - self.damping) / (n as f64);
            }
            ranks = new_ranks;
        }
        ranks
    }
}

fn main() {
    let graph = vec![
        vec![1,2],
        vec![0],
        vec![0, 3],
        vec![0],
        vec![0, 1],
    ];

    let names = vec!["ESPN", "NFL", "NBA", "UFC", "MLB"];
    let pagerank = PageRank::new(0.85, 100);
    let ranks = pagerank.rank(&graph);

    for (i, rank) in ranks.iter().enumerate() {
        println!("The pagerank of {} is {}", names[i], rank);
    }

}