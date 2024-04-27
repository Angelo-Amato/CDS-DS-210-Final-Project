use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;

//types and struct definitions to make the graph from the txt file
pub type Vertex = usize;
type ListOfEdges = Vec<(Vertex, Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

pub struct Graph {
    pub n: usize, // vertex labels in {0,...,n-1}
    pub outedges: AdjacencyLists,
}

// methods for the Graph struct
impl Graph {
    // takes a ListOfEdges and adds the edges to the graph
    fn add_directed_edges(&mut self, edges: &ListOfEdges) {
        for (u, v) in edges {
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
    pub fn create_directed(n: usize, edges: &ListOfEdges) -> Graph {
        let mut g = Graph {
            n,
            outedges: vec![vec![]; n],
        };
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g
    }
}

// creates a key from a tsv file that maps article names to numbers 0 to n-1 where n is the number of pages
// assumes the file is in the format of a tsv file with the first column being the article name
pub fn create_key(path: &str) -> HashMap<String, usize> {
    let mut key = HashMap::new();

    //read file
    let file = File::open(path).expect("Could not open file");
    let mut coded_number: usize = 0;
    let buf_reader = std::io::BufReader::new(file).lines();
    //page name and number stored in a HashMap
    for line in buf_reader {
        let line_str = line.expect("Error reading").trim().to_string();
        key.insert(line_str, coded_number);
        coded_number += 1;
    }
    key
}
// creates a reverse key where the number is the key and the article name is the value
// assumes there is no duplicate values
pub fn reverse_key(key: &HashMap<String, usize>) -> HashMap<usize, String> {
    let mut reverse_key = HashMap::new();
    for (k, v) in key.iter() {
        reverse_key.insert(*v, k.clone());
    }
    reverse_key
}

// creates a graph from a tsv file with the key mapping article names to numbers
// assumes the file is in the format of a tsv file with the first column being the start node and the second column being the end node
pub fn numbered_nodes_graph(path: &str, key: HashMap<String, usize>) -> Graph {
    // creates an edge list for the outedges of the graph
    let mut edge_list: Vec<(Vertex, Vertex)> = Vec::new();
    // reads the file and goes through line by line
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        // creates a 2 string vector from each line in the file
        let edge: Vec<&str> = line_str.trim().split('\t').collect();
        // gets the number of the article from the key
        let edge_start = key.get(edge[0]).expect("Error getting key");
        let edge_end = key.get(edge[1]).expect("Error getting key");
        // adds the edge to the edge list
        edge_list.push((*edge_start, *edge_end));
    }
    // creates a graph from the edge list and length of the key (number of pages in the graph)
    Graph::create_directed(key.len(), &edge_list)
}
