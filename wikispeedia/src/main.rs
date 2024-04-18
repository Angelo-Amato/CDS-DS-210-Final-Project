mod graph_read;
mod bfs;
fn main() {
    use graph_read::*;
    use bfs::get_average_distance;

    let key = create_key("articles.tsv");
    let reverse = reverse_key(&key);
    let wiki_graph = numbered_nodes_graph("links.tsv", key);
    for x in 0..wiki_graph.n{
        let test = get_average_distance(x as Vertex, &wiki_graph);
        let page_name = reverse.get(&x).unwrap();
        println!("Average distance from node {:?}: {:?}", page_name, test);
    }
 
}
