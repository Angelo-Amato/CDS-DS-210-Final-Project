mod bfs;
mod graph_read;
fn main() {
    // imports the modules to change some names with non standard characters
    use percent_encoding::percent_decode_str;

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
    for x in 0..wiki_graph.n {
        // calculate the average distance from  node x  to all other nodes
        let ave_dist = get_average_distance(x as Vertex, &wiki_graph);

        // add the average distance to a vector to find average of graph
        graph_ave.push(ave_dist);

        // formats the printing of the article name and the average distance to be more readable
        let page_name = reverse.get(&x).unwrap();
        let decoded_name = percent_decode_str(page_name)
            .decode_utf8()
            .unwrap()
            .to_string()
            .replace("_", " ");
        let dashes = "-".repeat(max_length - decoded_name.chars().count());
        println!("{}{}{}", decoded_name, dashes, ave_dist)
    }

    // calculates the average distance from  all the nodes to all other nodes
    let total_ave = graph_ave.iter().sum::<f64>() / graph_ave.len() as f64;
    println!("The average distance from any given node to any other given node is {}", total_ave);
}
