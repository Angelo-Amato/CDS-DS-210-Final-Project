mod graph_read;
fn main() {
    use graph_read::*;
    let key = create_key("articles.tsv");
    let wiki_graph = numbered_nodes_graph("links.tsv", key);
}