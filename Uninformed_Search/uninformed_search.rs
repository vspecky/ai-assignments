/**
 * Name: Shashank Attarde
 * Branch: TY B.Tech Comp
 * Roll No: 015
 *
 * AI Lab Assignment: Application of Uninformed Search
 *
 * Problem Chosen: N-Queens Problem
 *
 * Algorithms Used:
 *    - DFS
 *    - DFS with Pruning
 *
 * Explanation of the Problem:
 * Given N Queens, our job is to place these queens on an NxN chessboard such that no two
 * queens can attack each other.
 *
 * Modes Implemented:
 *  - Brute Force
 *  - Brute Force with Pruning
 *  - Row-wise optimized Brute Force
 *  - Row-wise optimized Brute Force with Pruning
 *  - Multi-Threaded Brute Force
 *  - Multi-Threaded Brute Force with Pruning
 *  - Multi-Threaded Row-wise optimized Brute Force
 *  - Multi-Threaded Row-wise optimized Brute Force with Pruning
 *
 * Some Observations given N Queens:
 *  - Depth of the tree is N.
 *  - Branching Factor of a Node at a depth of 'm' is (N^2 - m) for Brute Force Search and 'm' for
 *    Row-wise optimized Brute Force Search.
 *  - Total number of Leaf Nodes in the tree are (N^2)! / ((N^2) - N)! for Brute Force Search and
 *    N^N for Row-wise optimized Brute Force Search.
 *
 * Usage:
 *   ./nqueens <N> <mode> <threading>
 *
 *   where <N> is the number of Queens N and is a positive integer
 *
 *   <mode> is the solving mode which can be one of the following :-
 *     'bf'  - Brute Force
 *     'rw'  - Row-wise optimized Brute Force
 *     'pbf' - Brute Force with Pruning
 *     'prw' - Row-wise optimized Brute Force with Pruning
 *
 *  <threading> is whether to use multithreading or not :-
 *     't' - Use multithreading
 *     'f' - Don't use multithreading
 */
use std::collections::HashSet;
use std::env;
use std::sync::mpsc;
use std::thread;
use std::vec::Vec;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 4 {
        eprintln!("Please provide N and the Solving Mode as an argument");
        return;
    }

    let n = match args[1].parse::<i8>() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("N should be a positive integer greater than 4");
            return;
        }
    };

    let mode = match args[2].as_str() {
        "bf" => Modes::BruteForce,
        "rw" => Modes::RowWise,
        "pbf" => Modes::PruneBruteForce,
        "prw" => Modes::PruneRowWise,
        _ => {
            eprintln!("Invalid mode. Allowed modes are bf, rw, pbf and prw");
            return;
        }
    };

    let threading = match args[3].as_str() {
        "t" => true,
        _ => false,
    };

    if n < 1 {
        eprintln!("N should be a positive integer");
        return;
    }

    let mut solver = Solver::new(n, mode);

    if threading {
        solver.solve_threaded();
    } else {
        solver.solve();
    }
}

pub struct State {
    queens: HashSet<(i8, i8)>,
}

impl State {
    pub fn new(queens: HashSet<(i8, i8)>) -> Self {
        Self { queens }
    }

    pub fn full(&self, n: i8) -> bool {
        self.queens.len() as i8 == n
    }

    pub fn add_queen_brute(&self, n: i8) -> Vec<Self> {
        if self.full(n) {
            return Vec::new();
        }

        let n_u = n as usize;

        let mut res = Vec::<Self>::with_capacity(n_u * n_u - self.queens.len());

        for y in 0..n {
            for x in 0..n {
                if self.queens.contains(&(x, y)) {
                    continue;
                }

                let mut child_queens = self.queens.clone();
                child_queens.insert((x, y));

                res.push(Self::new(child_queens));
            }
        }

        res
    }

    pub fn add_queen_rowwise(&self, n: i8) -> Vec<Self> {
        if self.full(n) {
            return Vec::new();
        }

        let mut res = Vec::<Self>::with_capacity(n as usize);

        let y = self.queens.len() as i8;

        for x in 0..n {
            let mut child_queens = self.queens.clone();
            child_queens.insert((x, y));

            res.push(Self::new(child_queens));
        }

        res
    }

    pub fn check_valid(&self, n: i8) -> bool {
        let n_u = n as usize;

        let mut xs = HashSet::<i8>::with_capacity(n_u);
        let mut ys = HashSet::<i8>::with_capacity(n_u);
        let mut rd = HashSet::<i8>::with_capacity(n_u);
        let mut ld = HashSet::<i8>::with_capacity(n_u);

        for (x, y) in self.queens.iter() {
            if xs.contains(x) || ys.contains(y) || rd.contains(&(x + y)) || ld.contains(&(x - y)) {
                return false;
            }

            xs.insert(*x);
            ys.insert(*y);
            rd.insert(*x + *y);
            ld.insert(*x - *y);
        }

        true
    }

    pub fn display(&self, n: i8) {
        for y in 0..n {
            for x in 0..n {
                print!(
                    "{}",
                    if self.queens.contains(&(x, y)) {
                        "Q "
                    } else {
                        "- "
                    }
                );
            }

            print!("\n");
        }

        println!("\n");
    }
}

pub enum Modes {
    BruteForce,
    RowWise,
    PruneBruteForce,
    PruneRowWise,
}

pub struct Solver {
    n: i8,
    stack: Vec<State>,
    prune: bool,
    add_fn: fn(&State, i8) -> Vec<State>,
}

impl Solver {
    pub fn new(n: i8, mode: Modes) -> Self {
        let n_u = n as usize;
        let (add_fn, prune): (fn(&State, i8) -> Vec<State>, bool) = match mode {
            Modes::BruteForce => (State::add_queen_brute, false),
            Modes::RowWise => (State::add_queen_rowwise, false),
            Modes::PruneRowWise => (State::add_queen_rowwise, true),
            Modes::PruneBruteForce => (State::add_queen_brute, true),
        };

        Self {
            n,
            add_fn,
            prune,
            stack: Vec::<State>::with_capacity(n_u * n_u),
        }
    }

    pub fn solve(&mut self) {
        self.stack
            .push(State::new(HashSet::<(i8, i8)>::with_capacity(
                self.n as usize,
            )));

        while self.stack.len() > 0 {
            let current = self.stack.pop().unwrap();

            if self.prune && !current.check_valid(self.n) {
                continue;
            }

            if current.full(self.n) {
                if current.check_valid(self.n) {
                    println!("Solution found :-");
                    current.display(self.n);
                    return;
                }

                //current.display(self.n);
            }

            self.stack.append(&mut (self.add_fn)(&current, self.n));
        }

        println!("No solution found");
    }

    pub fn solve_threaded(&mut self) {
        if self.n < 4 {
            println!("No solution");
            return;
        }

        let initial = State::new(HashSet::<(i8, i8)>::with_capacity(self.n as usize));

        let (tx, rx) = mpsc::channel::<State>();

        for s in (self.add_fn)(&initial, self.n).drain(..) {
            let tx_clone = tx.clone();
            let n_u = self.n as usize;
            let n = self.n;
            let prune = self.prune;
            let add_fn = self.add_fn;
            thread::spawn(move || {
                let mut stack = Vec::<State>::with_capacity(n_u * n_u);
                stack.push(s);

                while stack.len() > 0 {
                    let current = stack.pop().unwrap();

                    if prune && !current.check_valid(n) {
                        continue;
                    }

                    if current.full(n) {
                        if current.check_valid(n) {
                            tx_clone.send(current).unwrap();
                            return;
                        }
                    }

                    let mut children = (add_fn)(&current, n);

                    stack.append(&mut children);
                }
            });
        }

        let solution = rx.recv().unwrap();
        println!("Solution found");
        solution.display(self.n);
    }
}
