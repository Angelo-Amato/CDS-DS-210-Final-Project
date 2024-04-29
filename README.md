# CDS-DS-210-Final-Project
>Angelo Amato
>
>1 May 2024
>
>Professor Kontothanassis

## About
The game Wikispeedia is an online game that involves navigating through the Wikipedia website towards a "target article" using only the hyperlinks on each  page. The player is supposed to do this in as little clicks, and as fast as possible. In this project, I will use the smaller(~4,600 articles) version of Wikipedia that the game uses. The Stanford Network Analysis Project (SNAP) has published a dataset of the links in this game.

In this project I am aiming to calculate the average number of clicks to navigate from one random page to another. In exploring the [Wikispeedia website](https://dlab.epfl.ch/wikispeedia/play/), the average number of clicks is about 4-10, with the record for some scenarios being even lower than that. I want to see if the records held for certain runs are only that high because people are missing hidden routes.  

## Code Description
I used a Cargo-based Rust project found in the wikispeedia folder of this repository. The project is broken down into three files: `graph_read.rs`, `bfs.rs`, and `main.rs`. The first file `graph_read.rs`, is primarily used to create the graph from the tsv files and convert them to a usable format. The `bfs.rs` file contains the helper function in order to calculate the average distance from any given node to all other nodes. Finally, the `main.rs` file imports the functions and types from the other two files and runs the actual calculations. It also formats the output so all of the results are inline with the actual article name. 

Below is a breakdown of the code in each file:

#### `graph_read.rs`
This file houses helper functions that read the tsv files containing the data. There were two files: `articles.tsv`, which contained the names of the articles in alphabetical order, and `links.tsv`, which contained a list of links that form the edges of the directed graph. 

* `Graph`
  *  a struct representing a directed graph
  *  contains attributes n (the number of nodes) and an outedges (an adjacency list representing the graph)
  *  contains several methods including a `create_directed()` which creates a Graph struct from a List of Edges
 
* `create_key`
  * a helper function to create key to turn the names of articles into numeric values for the graph
  * takes a file path as input (assumes it is a tsv file
  * returns a HashMap, with the article names as keys and the corresponding numbers as values
  * values will range from 0 to n-1, with n being the number of articles

* `reverse_key`
  * a helper function that takes a HashMap key and returns a Hashmap with the keys as values and vice versa
  * assumes that hte values are non-repeating numbers from 0 to n-1, with n being the number of articles
  * serves as a way to get the article names from the numeric values in the graph.
 
* `numbered_nodes_graph`
  * takes a HashMap key and a tsv file path as arguments
  * returns a graph representing the names articles in the tsv files
  * uses the HashMap key to change the string names of the articles to numerics
  * assumes the tsv file is two columns, representing the starting and ending nodes of a list of edges, with each new line being a different edge

#### `bfs.rs`
This file contains the function that runs the breadth first search algorithm used to calculate the average distance from a given node to all  *connected* nodes in the graph.

* `get_average_distance`
  * takes a starting vertex and a graph and calculates the distance to all other connected nodes in the graph
  * gets the distance to every reachable node in a vector, then gets the average of all values in the vector
  * does not take into account vectors that may not be connected

#### `main.rs`
This file calls some of the helper functions listed above and calculates the average distance for all starting nodes in the graph. It also prints the outputs as all of the articles with their average distance to all other nodes.

* `decode_names`
  * takes the percent encoded name of the article (which is how the data is originally formatted)
  * changes the article names to ones with special characters
  * also replaces underscores with spaces
    
* `main`
  * creates a key and and a reverse key from the `articles.tsv` file
  * uses the key and the `links.tsv` file to make a graph of the Wikipedia articles
  * Prints headers for the output
  * Iterates through each of the nodes in the graph
      * Gets the average distance to all other connected articles and adds it to a vector
      * Uses the reverse key to get the article name
      * Changes all percent encodings to special characters
      * Prints the article name with the average distance to all other connected articles
  * Uses the vector and finds the average of the average distances, the maximum, and the minimum
  
#### Tests
* Test 1
  * Tests the `get_average_distance` function with a 4 node graph of nodes all in a line
  * Gets the average distance of all nodes to the 0 node (should be 2.0)
  * <img width="445" alt="image" src="https://github.com/Angelo-2231/CDS-DS-210-Final-Project/assets/64280204/22e6c88b-ced8-48d5-b4ec-b86761c67ace">

 
* Test 2
  * Tests the `get_average_distance` function with a slightly more complex graph with edges in differing directions
  * Gets the average distance of all nodes to the 0 node (should be 1.75)
  * <img width="445" alt="image" src="https://github.com/Angelo-2231/CDS-DS-210-Final-Project/assets/64280204/c9fa9881-18a8-42a8-84ba-8906661a3e69">

* Test 3
  * Tests the `reverse_key` function
  * makes sure that the function properly switches the keys and values (assuming no repeated keys) 


## Output and Observations
The full output of `main.rs` can be found in the [full_output.md](full_output.md) file. An abridged version of the output can be found below:

```text
The average distance from any given node to any other given node is 2.8090150287284805
The maximum distance from any given node to any other given node is 4.5103193569411255
The minimum distance from any given node to any other given node is 0
```
Taking this in the context of the Wikispeedia game, it takes, on average approximately 2.8 clicks to reach the target article. The maximum value was approximately 4.5, meaning that at most, an article has an average distance of 4 - 5 clicks from any other given article. This may mean that there is a significant amount of human error when playing the game, especially since some of the "record" minimum number of clicks for certain routes can be 10 or greater.

One interesting ovbservation is that there are several articles with an average distance of 0 to any other given article. This is an interesting phenomenon known as ["dead-end pages"](https://en.wikipedia.org/wiki/Wikipedia:Deadend_pages#:~:text=A%20dead%2Dend%20page%20is,easy%20cross%2Dreferencing%20of%20information.), which are ones with no outgoing links. This means their default distance to all other values would default to 0, since that is the distance to itself given the way the program was written. One example of this is the Wikipedia page for Duchenne muscular dystrophy (pictured below). There are no outgoing links on this page found in the game aside from the ones the Wikispeedia game puts at the end of every article in the game. It is important to note however that the article for Duchenne muscular dystrophy is no longer a dead-end page, as links have been added to it since the Wikispeedia game was created.

<img width="1425" alt="image" src="https://github.com/Angelo-2231/CDS-DS-210-Final-Project/assets/64280204/9ebb4da6-8381-46d2-a719-02b9df984e34">


## Sources
1. [Stanford Network Analysis Project (SNAP) Dataset](https://snap.stanford.edu/data/wikispeedia.html)
2. [Wikispeedia](https://dlab.epfl.ch/wikispeedia/play/)
3. [Wikipedia:Dead-end pages](https://en.wikipedia.org/wiki/Wikipedia:Deadend_pages#:~:text=A%20dead%2Dend%20page%20is,easy%20cross%2Dreferencing%20of%20information.)
4. [Current Wikipedia article for Duchenne muscular dystrophy](https://en.wikipedia.org/wiki/Duchenne_muscular_dystrophy)
5. [Vistualizations of Graphs Sandboxes](https://csacademy.com/app/graph_editor/)
6. [ChatGPT Conversation 1](https://chat.openai.com/share/960d84a6-8bd0-435a-9f49-1ed69f4ae4a4)
7. [ChatGPT Conversation 2](https://chat.openai.com/share/d093b3ba-81fd-48b3-a603-1fa45248bf54)
