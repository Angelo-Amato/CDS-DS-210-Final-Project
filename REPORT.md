# CDS-DS-210-Final-Project
>Angelo Amato
>
>1 May 2024
>
>Professor Kontothanassis

## About
The game Wikispeedia is an online game that involves navigating through the Wikipedia website towards a "target article" using only the hyperlinks on each  page. The player is supposed to do this in as little clicks, and as fast as possible. In this project, I will use the smaller(~4,600 articles) version of Wikipedia that the game uses. The Stanford Network Analysis Project (SNAP) has published a dataset of the links in this game.

In this project I am aiming to calculate the average number of clicks to navigate from one random page to another. In exploring the (Wikispeedia website)[https://dlab.epfl.ch/wikispeedia/play/], the average number of clicks is about 4-10, with the record for some scenarios being even lower than that. I want to see if the records held for certain runs are only that high because people are missing hidden routes.  

## Code Description
The project is broken down into three files: `graph_read.rs`, `bfs.rs`, and `main.rs`. The first file `graph_read.rs`, is primarily used to create the graph from the tsv files and convert them to a usable format. The `bfs.rs` file contains the helper function in order to calculate the average distance from any given node to all other nodes. Finally, the `main.rs` file imports the functions and types from the other two files and runs the actual calculations. It also formats the output so all of the results are inline with the actual article name. 

Below is a breakdown of the code in each file:

#### `graph_read.rs'

#### `bfs.rs`

#### `main.rs`

## Output and Observations

## Sources
1. (Stanford Network Analysis Project (SNAP) Dataset)[https://snap.stanford.edu/data/wikispeedia.html]
2. (Wikispeedia)[https://dlab.epfl.ch/wikispeedia/play/]
3. (ChatGPT Conversation 1)[https://chat.openai.com/share/960d84a6-8bd0-435a-9f49-1ed69f4ae4a4]
4. (ChatGPT Conversation 2)[https://chat.openai.com/share/d093b3ba-81fd-48b3-a603-1fa45248bf54]
