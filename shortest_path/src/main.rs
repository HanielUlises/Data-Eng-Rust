use petgraph::algo::dijkstra;
use petgraph::prelude::*;

fn main() {
    let mut graph = Graph::<&str, u32, Undirected>::new_undirected();
    
    let western_wall = graph.add_node("Western Wall");
    let dome_of_the_rock = graph.add_node("Dome of the Rock");
    let mahane_yehuda = graph.add_node("Mahane Yehuda Market");
    let tel_aviv_port = graph.add_node("Tel Aviv Port");
    let old_jaffa = graph.add_node("Old Jaffa");

    graph.extend_with_edges([
        (western_wall, dome_of_the_rock, 1),
        (western_wall, mahane_yehuda, 3),
        (western_wall, tel_aviv_port, 7),
        (dome_of_the_rock, mahane_yehuda, 2),
        (dome_of_the_rock, tel_aviv_port, 6),
        (mahane_yehuda, tel_aviv_port, 5),
        (tel_aviv_port, old_jaffa, 1),
    ]);

    let node_map = dijkstra(&graph, western_wall, Some(old_jaffa), |e| *e.weight());

    if let Some(distance) = node_map.get(&old_jaffa) {
        println!(
            "The shortest distance from Western Wall to Old Jaffa is {} km", distance
        );
    } else {
        println!("No route found from Western Wall to Old Jaffa");
    }
}