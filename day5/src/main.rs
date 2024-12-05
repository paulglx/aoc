use std::{
    collections::{HashMap, VecDeque},
    fs,
};

use petgraph::{
    graph::{DiGraph, NodeIndex},
    matrix_graph::Zero,
};

fn check_update(
    update: &str,
    nodes: &HashMap<&str, NodeIndex>,
    graph: &DiGraph<&str, &str>,
) -> bool {
    // 2. Check updates
    // check if one update, like "34, 57, 61, ..." is valid
    // an update is valid if the path A -> B -> (...) -> Z is valid
    // therefore, we walk node by node and check if the path is possible

    let mut all_nodes: VecDeque<&str> = update.split(",").collect();

    let mut current_node = *nodes.get(all_nodes.pop_front().unwrap()).unwrap();
    let mut next_node = *nodes.get(all_nodes.pop_front().unwrap()).unwrap();

    let mut result = true;

    while !all_nodes.len().is_zero() {
        match graph.neighbors(current_node).find(|n| *n == next_node) {
            Some(node) => current_node = node,
            None => {
                result = false;
            }
        }
        next_node = *nodes.get(all_nodes.pop_front().unwrap()).unwrap();
    }

    result
}

fn main() {
    // Concept : this is a graph problem
    // 1. Create a graph linking page numbers (node) to all possible succeeding nodes
    // 2. Check validity of update by checking if graph path is possible
    // 3. Filter updates with this criteria
    // 4. Get the middle value of each update and sum them

    let file_content = fs::read_to_string("input.txt").unwrap();

    // 1. COMPUTE RULES : Create graph

    let rules = file_content.split("\n\n").next().unwrap();

    let mut graph = DiGraph::<&str, &str>::new();
    // put possible graph nodes in a hashmap : "value" -> nodeindex
    // this will be useful later to get nodeindex by value in o(1) time
    let mut nodes: HashMap<&str, NodeIndex> = HashMap::new();
    for n in rules.lines().map(|line| line.split("|").next().unwrap()) {
        if !nodes.contains_key(n) {
            nodes.insert(n, graph.add_node(n));
        }
    }

    // Create edges in the graph
    // Edges are created by NodeIndex, therefore a full reference is needed

    for rule in rules.lines() {
        let (from, to): (&str, &str) = rule.split_once("|").unwrap();

        let node_from = *nodes.get(from).unwrap();
        let node_to = *nodes.get(to).unwrap();

        graph.add_edge(node_from, node_to, "");
    }

    // 2-3. CHECK UPDATES and FILTER
    let updates = file_content.split("\n\n").nth(1).unwrap().lines();

    let filtered_updates = updates
        .filter(|&update| check_update(update, &nodes, &graph))
        .map(|line| line.split(",").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    //
    // 4. sum middle values
    let result: u32 = filtered_updates
        .into_iter()
        .map(|update| update[update.len() / 2].parse::<u32>().unwrap())
        .sum();

    println!("{}", result);
}
