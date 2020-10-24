# Tic Tac Toe AI with Minimax
This is a simple implementation of an AI for Tic Tac Toe that uses the famous
Minimax algorithm, for an assignment for my AI Course at College.

## Scoring Scheme
- 1000 for 3 identical symbols in a row. (This is the 'Win/Lose' condition)
- 100 for two symbols and a blank space in a row.
- 10 for one symbol and two blank space in a row.

Total score of a state = `O` score - `X` score

## Miscellaneous Notes
- Tic Tac Toe is one of the few games where it is feasible for the Minimax algorithm
to explore all nodes up till the leaf node due to low depth and a decreasing
branching factor at each level.
- Amount of nodes to explore when opponent goes first = 9!
- Amount of nodes to explore when player goes first = 8!
