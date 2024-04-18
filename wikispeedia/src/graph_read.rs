use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;

//types and struct definitions to make the graph from the txt file
type Vertex = usize;
type ListOfEdges = Vec<(Vertex,Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

//#[derive(Debug)]
pub struct Graph {
    pub n: usize, // vertex labels in {0,...,n-1}
    pub outedges: AdjacencyLists,
}

// methods for the Graph struct
impl Graph {
    // takes a ListOfEdges and adds the edges to the graph
    fn add_directed_edges(&mut self,edges:&ListOfEdges) {
        for (u,v) in edges {
            self.outedges[*u].push(*v);
        }
    }
    // sorts the adjacency lists
    fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }
    // creates a graph from a ListOfEdges and number of nodes 
    fn create_directed(n:usize,edges:&ListOfEdges) -> Graph {
        let mut g = Graph{n,outedges:vec![vec![];n]};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g                                        
    }
}

pub fn create_key(path:&str) -> HashMap<String, usize>{
    let mut key = HashMap::new();
    let file = File::open(path).expect("Could not open file");
    let mut coded_number: usize = 0;
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader{
        let line_str = line.expect("Error reading").trim().to_string();
        key.insert(line_str, coded_number);
        coded_number += 1;
    }
    key 
}

pub fn numbered_nodes_graph(path:&str, key: HashMap<String, usize>) -> Graph{
    let mut edge_list: Vec<(Vertex, Vertex)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader{
        let line_str = line.expect("Error reading");
        let edge: Vec<&str> = line_str.trim().split('\t').collect();
        let edge_start = key.get(edge[0]).expect("Error getting key");
        let edge_end = key.get(edge[1]).expect("Error getting key");
        edge_list.push((*edge_start, *edge_end));
    }
    Graph::create_directed(key.len(), &edge_list)
    
}