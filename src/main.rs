use petgraph::algo::dijkstra;
use petgraph::dot::Dot;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::{Graph, Undirected};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;


fn main() {
    let (label_map, graph) = nodes_edges();
    let (start_label, end_label) = get_start_end();

    if let (Some(&start_index), Some(&end_index)) =
        (label_map.get(&start_label), label_map.get(&end_label))
    {
        let shortest_path_map: HashMap<NodeIndex, usize> =
            dijkstra(&graph, start_index, Some(end_index), |_| 1);
        println!("shortest path map{shortest_path_map:?}");
        let mut path: Vec<NodeIndex> = Vec::new();
        let mut current_index: NodeIndex = end_index;

        // reverse tracking from the end node to the start node
        while current_index != start_index {
            println!("cur{current_index:?}");
            println!("star{start_index:?}");
            path.push(current_index);
            if let Some(next) = shortest_path_map.get(&current_index) {
                if *next == current_index.index() {
                    eprintln!("Error: cycle detected or path reconstruction failed.");
                    break;
                }
                current_index = NodeIndex::new(*next);
            } else {
                eprintln!("No valid path found from {start_label} to {end_label}.");
                break;
            }
        }

        if current_index == start_index {
            path.push(start_index);
            path.reverse();

            let path_labels: Vec<String> = path
                .iter()
                .map(|&node_index| graph.node_weight(node_index).unwrap().clone())
                .collect();
            println!(
                "Shortest path from {} to {}: {}",
                start_label,
                end_label,
                path_labels.join(" -> ")
            );
        } else {
            println!("No valid path could be reconstructed.");
        }
    } else {
        eprintln!("Invalid start or end label.");
    }
}

fn get_start_end() -> (String, String) {
    let mut start_label: String = String::new();
    let mut end_label: String = String::new();

    print!("Enter start label: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut start_label).unwrap();
    let start_label: String = start_label.trim().to_owned();

    print!("Enter end label: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut end_label).unwrap();
    let end_label: String = end_label.trim().to_owned();
    (start_label, end_label)
}
fn nodes_edges() -> (
    HashMap<String, NodeIndex>,
    Graph<String, String, Undirected>,
) {
    let mut graph: petgraph::Graph<String, String, petgraph::Undirected> =
        UnGraph::<String, String>::with_capacity(4, 4);
    let mut node_map: HashMap<String, NodeIndex> = HashMap::new();
    let mut label_map: HashMap<String, NodeIndex> = HashMap::new();
    let mut edge_labels: HashMap<(NodeIndex, NodeIndex), String> = HashMap::new();

    println!("Reading nodes from file...");
    for line in read_lines("nodes.txt").unwrap() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let (node_value, node_name) = (parts[0].to_string(), parts[1].to_string());
            let node_index = graph.add_node(node_name.clone());
            node_map.insert(node_value.clone(), node_index);
            label_map.insert(node_name.clone(), node_index);
            println!("Added node: {node_value} with index: {node_index:?}");
        } else {
            eprintln!("Invalid node line format: {line}");
        }
    }

    println!("Reading edges from file...");
    for line in read_lines("edges.txt").unwrap() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 3 {
            let (node1, node2, edge_label) = (
                parts[0].to_string(),
                parts[1].to_string(),
                parts[2].to_string(),
            );

            if let (Some(&index1), Some(&index2)) = (node_map.get(&node1), node_map.get(&node2)) {
                graph.add_edge(index1, index2, edge_label.clone());
                graph.add_edge(index2, index1, edge_label.clone());
                edge_labels.insert((index1, index2), edge_label.clone());
                edge_labels.insert((index2, index1), edge_label.clone());
                println!("Added edge between: {node1} and {node2} with label: {edge_label}");
            } else {
                eprintln!("Node index not found for edge: {node1} - {node2}");
            }
        } else {
            eprintln!("Invalid edge line format: {line}");
        }
    }

    println!("Number of nodes: {}", graph.node_count());
    println!("Number of edges: {}", graph.edge_count());

    let dot_output: String = Dot::new(&graph).to_string();
    println!("{dot_output}");
    (label_map, graph)
}
fn read_lines<P>(filename: P) -> io::Result<impl Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().map_while(Result::ok))
}
