# SuperRusty
Super Rusty is a Chess AI written in rust.

# To-do:

## Chess framework
- ~~Define pieces~~
- ~~Define board~~
- Define moves of pieces
  - Pawns (and en passant)
  - Knights
  - Rooks (and Castling)
  - Queen
  - Bishops
  - King
- Implement pawn promotion
- Check for check
- Check for mate
- Check validity of a move
- Support chess notation input

## AI
- Define evaluation function
- Define possible moves
- Create basic negamax implementation


## Get all possible moves:
1. Iterate over each piece type (pawns, knights, bishops, rooks, queens, and kings) and generate a list of legal moves for that piece type. You can do this by using bitwise operations and lookup tables to compute the squares that the pieces can legally move to, based on their type and the current board state.

2. For each piece type, iterate over the bitboard for that piece type, and use bit-scanning or magic bitboards (as described in the previous answer) to extract the legal moves for each individual piece.

3. Add the legal moves for each piece to a list of all legal moves.

4. When you have processed all the piece types, the list of legal moves will contain all the possible (and legal) moves for the current position on the board.