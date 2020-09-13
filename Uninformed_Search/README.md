# Uninformed Search (N-Queens Problem)

The N-Queens Problem is a famous problem where you are given N chess queens
and tasked with placing them on an NxN chessboard such that no pair of queens
can attack each other.

### Algorithms Used
- Depth First Search
- Depth First Search with Pruning

### Methods Implemented
- Brute Force
- Brute Force with Pruning
- Row-wise optimized Brute Force
- Row-wise optimized Brute Force with Pruning
- Multi-Threaded Brute Force
- Multi-Threaded Brute Force with Pruning
- Multi-Threaded Row-wise optimized Brute Force
- Multi-Threaded Row-wise optimized Brute Force with Pruning

### Usage
```
./nqueens <N> <mode> <threading>
```
Where `<N>` is the number of queens.  
`<mode>` is the solving mode and can be one of the following :-
- **bf**: Brute Force
- **rw**: Row-wise optimized Brute Force
- **pbf**: Brute Force with Pruning
- **prw**:  Row-wise optimized Brute Force with Pruning
`<threading>` is whether to use multithreading or not :-
- **t** - Use
- **f** - Nah
