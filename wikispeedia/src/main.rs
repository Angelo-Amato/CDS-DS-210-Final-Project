mod bfs;
mod graph_read;
// changes the names of necessary functions to be more readable
fn decode_name(percent_name: &str) -> String {
    let decoded_name = percent_encoding::percent_decode_str(percent_name)
        .decode_utf8()//adds special characters
        .unwrap()//unwraps the result
        .to_string()//converts to string
        .replace("_", " ");// replaces underscores with spaces 
    decoded_name
}
fn main() {

    // import the functions from the modules in graph_read.rs and bfs.rs
    use bfs::get_average_distance;
    use graph_read::*;

    // creates a HashMap to change the name of the article from a number from 0 to n-1
    let key = create_key("articles.tsv");

    // creates a reverse key where the number is the key and the article name is the value
    let reverse = reverse_key(&key);

    // creates a graph from a tsv file with the key mapping article names to numbers
    let wiki_graph = numbered_nodes_graph("links.tsv", key);

    // goes through each node and calculate the average distance from that node to all other nodes
    let max_length = 100;
    let mut graph_ave = Vec::<f64>::new();
    //prints headers for the output
    println!("Article Name{}Average Distance to Other Articles", "-".repeat(max_length - "Article Name".chars().count()));
    println!("{}",":".repeat(max_length + 34));
    for x in 0..wiki_graph.n {
        // calculate the average distance from  node x  to all other nodes
        let ave_dist = get_average_distance(x as Vertex, &wiki_graph);

        // add the average distance to a vector to find average of graph
        graph_ave.push(ave_dist);

        // formats the printing of the article name and the average distance to be more readable
        let page_name = reverse.get(&x).unwrap();
        let decoded_name = decode_name(page_name);
        let dashes = "-".repeat(max_length - decoded_name.chars().count());
        println!("{}{}{}", decoded_name, dashes, ave_dist)
    }

    // calculates the average distance from  all the nodes to all other nodes
    let total_ave = graph_ave.iter().sum::<f64>() / graph_ave.len() as f64;
    println!("The average distance from any given node to any other given node is {}", total_ave);


    // calculates the maximum average distance  
    let max_ave = graph_ave.iter().fold(0.0, |a: f64, &b| a.max(b));
    println!("The maximum distance from any given node to any other given node is {}", max_ave);

    // calculates the minimum average distance
    let min_ave = graph_ave.iter().fold(100000.0, |a: f64, &b| a.min(b)); //uses a starting value that is higher than the number of nodes to prevent errors
    println!("The minimum distance from any given node to any other given node is {}", min_ave);

}

//tests
#[cfg(test)]
mod tests {
    use crate::bfs::*;
    use crate::graph_read::*;
    use std::collections::HashMap;

    //tests the get_average_distance function for a smaller graph
    #[test]
    fn test_get_average_distance_line_graph() {
        let mut edges = Vec::new();
        edges.push((0, 1));
        edges.push((1, 2));
        edges.push((2, 3));
        
        let graph = Graph::create_directed(4, &edges);
        assert_eq!(get_average_distance(0, &graph), 2.0);// should be 2
    }

    #[test]
    fn test_get_average_distance_complex_graph(){
        let mut edges = Vec::new();
        edges.push((0, 1));
        edges.push((1, 2));
        edges.push((2, 1));
        edges.push((2, 3));
        edges.push((3, 4));
        edges.push((3, 0));
        edges.push((0, 4));

        let graph = Graph::create_directed(5, &edges);
        assert_eq!(get_average_distance(0, &graph), 1.75);//should be 7/4
    }
    
    //tests the reverse_key function
    #[test]
    fn test_reverse_key() {
        let mut key = HashMap::new();
        key.insert("a".to_string(), 0);
        key.insert("b".to_string(), 1);
        key.insert("c".to_string(), 2);
        let reverse = reverse_key(&key);
        assert_eq!(reverse.get(&0).unwrap(), "a");
        assert_eq!(reverse.get(&1).unwrap(), "b");
        assert_eq!(reverse.get(&2).unwrap(), "c");
    }
}