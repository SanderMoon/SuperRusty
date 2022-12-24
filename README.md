# SuperRusty
Super Rusty is a Chess AI written in rust.

Open issues can be viewed on the [Github Projects](https://github.com/users/SanderMoon/projects/1) page or on the [issues](https://github.com/SanderMoon/SuperRusty/issues) page. 

Be sure to send me a message if you want to contribute! 
If there is interest from the community, a discord server can be sey up. 

## Project outline

This is mainly a hobby projects to get into the Rust programming language, as well as to learn about advanced chess engines.
The outline of the project is as follows:

- Everything will be built from scratch, including the chessboard representation and the AI.
- The chessboard representation is created using [bitboards](https://www.chessprogramming.org/Bitboards)
- The bit board functions for move and attack generation should be heavily unit tested and commented, as debugging these are hard.
- The first version of the AI will use a simple tree-search algorithm like [Negamax AB search](https://en.wikipedia.org/wiki/Negamax) or [Monte Carlo Tree Search (MCTS)](https://en.wikipedia.org/wiki/Monte_Carlo_tree_search).
- Based on the choice of tree search algorithm, a neural network will be added to either replace the evaluation function in a Negamax implementation like [NNUE](https://www.chessprogramming.org/NNUE), or one that replaces random simulation with an evaluation score in an MCTS implementation. 
