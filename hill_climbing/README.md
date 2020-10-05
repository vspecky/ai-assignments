# Hill Climbing Algorithm for TSP
Implementation of the Hill Climbing Algorithm for the Travelling Salesperson Problem. In TSP, You are given N cities and your job is to find the shortest path that goes through all cities.
The brute force approach to this problem has a time complexity of O(n!) which is extremely bad.

To optimize the process, we can apply Hill Climbing to get a pretty good estimate of the solution.

## Procedure
- We init the algorithm by picking a random path without any initial calculations.
- We look at the neighbors of the path. The neighbors depend on the operator chosen. You can see an explanation of the two operator options below.
- Any neighbor whose path length is shorter than the path length of the current path replaces the current path as the new best path.
- If we found a new best path, we look at its neighbors and repeat the process.
- If no new best path is found, the hill climbing terminates.

The problem that arises is, there is no guarantee that running the algorithm once will give us the shortest possible path. A lot of the time, the hill climbing algorithm may get stuck at a peak or a plateau.
To address this, we run the algorithm over and over for 'M' iterations, and keep track of the global shortest path. The larger M is, the better the chance of us reaching the global minimum, since each
generation begins with a random path.

## Operators
There are two operators implemented in this implementation.

### Reversal Operator
Using this operator, we simply reverse a subset of the path. For example, consider the follwing path
```
A -> B -> C -> D -> E -> F -> G -> H -> A
```
Lets say we want to reverse the subset starting at `C` and ending at `F` i.e. `C D E F`.

On reversal we end up with the following path :-
```
A -> B -> F -> E -> D -> C -> G -> H -> A
```

Using this operator, we look at all such possible reversals (without including the origin node `A`) to get all neighbors of the path.

### Swap Operator
Self explanatory. Two nodes in the path are chosen and swapped. Once again, with this, we look at all possible swaps without including the origin node to get all neighbors of the path.


## Usage
You can edit `cities.tsp` to define your own cities. Each city must be on a separate line and should follow the format `NAME, X, Y` (the amount of spaces doesn't really matter).

Or you can run the `randomize_cities.py` script and it will generate 26 random cities for you, named `A-Z` and lying within the range `[0, 100)` on both axes.

To learn about script parameters, just run the script once without any parameters and it will tell you.
