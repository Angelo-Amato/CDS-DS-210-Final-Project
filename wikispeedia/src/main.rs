mod graph_read;
mod bfs;
fn main() {
    use graph_read::*;
    use bfs::get_average_distance;

    let key = create_key("articles.tsv");
    let reverse = reverse_key(&key);
    let wiki_graph = numbered_nodes_graph("links.tsv", key);
}