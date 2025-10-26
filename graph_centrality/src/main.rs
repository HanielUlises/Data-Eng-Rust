use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::Direction;
use std::fmt;

#[derive(Debug)]

struct Fighter{
    name : String
}

impl Fighter{
    fn new (name :&str) -> String{
        Self{
            name : name.to_string(),
        }
    }
}

impl fmt::Display for Fighter{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.name)
    }
}

fn add_edge(graph: &mut UnGraph<&Fighter, f32>, nodes: &[NodeIndex], a: usize, b: usize){
    graph.add_edge(nodes[a], nodes[b], 1.0);
}

fn main (){
    let mut graph = UnGraph::new_undirected();
    let fighters = [
        Fighter::new("Dustin Poirier"),
        Fighter::new("Khabib Nurmagomedov"),
        Fighter::new("Jose Aldo"),
        Fighter::new("Conor McGregor"),
        Fighter::new("Nate Diaz")
    ];

    let fighter_nodes: Vec<NodeIndex> = fighters.iter()
                                        .map(|fighter|graph.add_node(fighter))
                                        .collect();

    add_edge(&mut graph, &fighter_nodes, 0, 1);
    add_edge(&mut graph, &fighter_nodes, 1, 3);
    add_edge(&mut graph, &fighter_nodes, 3, 0);
    add_edge(&mut graph, &fighter_nodes, 3, 2);
    add_edge(&mut graph, &fighter_nodes, 3, 4);
    add_edge(&mut graph, &fighter_nodes, 0, 4);
    add_edge(&mut graph, &fighter_nodes, 2, 4);

    for(i, &node) in fighter_nodes.iter().enumerate(){
        let name = &fighters[i].name;
        let degree = graph.edges_directed(node, Direction::Outgoing).count as f32;
        let closeness = 1.0/degree;
        println!("The closeness centrality of {} is {}", name, closeness);
    }
    
}