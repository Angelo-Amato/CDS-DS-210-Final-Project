use crate::graph_read::*;
use std::collections::VecDeque;
use std::iter::Iterator;

// takes a graph and a starting node
// returns the average distance from the starting node to all other nodes
//adapted from function used in lecture
pub fn get_average_distance(start: Vertex, graph: &Graph) -> f64 {
    let mut distance: Vec<Option<u32>> = vec![None; graph.n];
    distance[start] = Some(0); // <= we know this distance
    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(start);
    while let Some(v) = queue.pop_front() {
        // new unprocessed vertex
        for u in graph.outedges[v].iter() {
            if let None = distance[*u] {
                // consider all unprocessed neighbors of v
                distance[*u] = Some(distance[v].unwrap() + 1);
                queue.push_back(*u);
            }
        }
    }
    // get the average distance to all points
    // using distance.len() instead of graph.n to account for the possibility of unreachable nodes
    // subtract 1 from the distance.len() to account for the starting node
    distance.iter().filter_map(|x| *x).sum::<u32>() as f64 / (distance.len() - 1) as f64
}
