# Chess Engine

A WIP chess engine made in rust with a GUI to allow the user to play against the AI.

## Try It

To play against the engine and test the GUI:
1. Download the source code and the executable chess.exe from the latest release
2. Place chess.exe in the root source code folder
3. Run chess.exe and try to win!

## How It Works

The engine uses a minmax ([negamax](https://www.chessprogramming.org/Negamax)) algorithm to determine the best move. Additionally, [Alpha-Beta Pruning](https://www.chessprogramming.org/Alpha-Beta) 
was used to significantly increase the performance of the search algorithm. In the current version of the engine, a search depth of 6 moves is used.

The board evaluation was done by summing the remaining piece values for both players and getting 'location value' for each piece using [Piece-Square Tables](https://www.chessprogramming.org/Piece-Square_Tables)

## To-Do

  - Allow the user to choose whether to play as white or black (as well as showing this visually - ie. reversing the board)
  - Incorporate UCI protocol in the engine to allow better compatibility with other GUI and testing software
  - Implement a transposition table to store previous searches
  - Use multiple piece-square tables for different stages of the game (opening, midgame, endgame)
  - Implement a [Quiescence Search](https://www.chessprogramming.org/Quiescence_Search) to the end of the main search algorithm to improve the safety and accuracy of moves
  - The "Play with your food" problem: if the engine can take free pieces before an inevitable checkmate it will sometimes do that instead of finding the 'quickest' checkmate 
  
## Built With

  - The move generation [chess library](https://docs.rs/chess/3.1.1/chess/) for rust
  - [ggez](https://docs.rs/ggez/0.5.1/ggez/): rust graphics library used to build the GUI
