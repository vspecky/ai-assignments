use std::io::{stdin, stdout, Error, Write};
use std::vec::Vec;

const MOVE: &str = "Invalid move. Moves to be entered as 'x <comma> y' where 0 <= x, y < 3";

const ARGS: &str =
    "Please specify whether you want to go first or second as an argument (1 = first, 2 = second)";

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        eprintln!("{}", ARGS);
        return;
    }

    let board: [[char; 3]; 3] = [['_', '_', '_'], ['_', '_', '_'], ['_', '_', '_']];

    match args[1].as_str() {
        "1" => go_first(board),
        "2" => go_second(board),
        _ => eprintln!("{}", ARGS),
    }
}

fn go_first(mut board: [[char; 3]; 3]) {
    loop {
        println!("\nCurrent State:");
        board.display();
        println!();

        while !player_move(&mut board) {}

        if check_win(&board) {
            break;
        }

        if check_draw(&board) {
            break;
        }

        opponent_move(&mut board);

        if check_lose(&board) {
            break;
        }
    }
}

fn go_second(mut board: [[char; 3]; 3]) {
    loop {
        opponent_move(&mut board);

        if check_lose(&board) {
            break;
        }

        if check_draw(&board) {
            break;
        }

        println!("\nCurrent State:");
        board.display();
        println!();

        while !player_move(&mut board) {}

        if check_win(&board) {
            break;
        }
    }
}

fn player_move(board: &mut [[char; 3]; 3]) -> bool {
    let inp = if let Ok(inp_str) = get_move_input() {
        inp_str
    } else {
        println!("There was a problem with the input");
        return false;
    };

    let (x, y) = match parse_move(inp) {
        Ok(mv) => mv,
        Err(msg) => {
            eprintln!("{}", msg);
            return false;
        }
    };

    if board[y][x] != '_' {
        eprintln!("That place is occupied, please select another move");
        return false;
    }

    board.make_move(x, y, Player::O);

    true
}

fn opponent_move(board: &mut [[char; 3]; 3]) {
    println!("Opponent is thinking...");

    let opp = board.minimax(8, false);

    let (x, y) = opp.to_move.unwrap();

    println!("Opponent's move: {}, {}", x, y);

    board.make_move(x, y, Player::X);
}

fn check_win(board: &[[char; 3]; 3]) -> bool {
    if board.get_score() == 1000 {
        println!("\n");
        board.display();
        println!("\nYou win!");
        true
    } else {
        false
    }
}

fn check_lose(board: &[[char; 3]; 3]) -> bool {
    if board.get_score() == -1000 {
        println!();
        board.display();
        println!("\nYou lose!");
        true
    } else {
        false
    }
}

fn check_draw(board: &[[char; 3]; 3]) -> bool {
    let mut draw = true;
    'outer: for row in board.iter() {
        for sym in row.iter() {
            if sym == &'_' {
                draw = false;
                break 'outer;
            }
        }
    }

    if draw {
        println!("\n");
        board.display();
        println!("\nDraw!");
    }

    draw
}

fn get_move_input() -> Result<String, Error> {
    let mut inp = String::new();

    print!("\nEnter your move (eg: 0, 2): ");
    stdout().flush()?;

    stdin().read_line(&mut inp)?;

    Ok(inp.split_whitespace().collect::<String>())
}

fn parse_move(move_str: String) -> Result<(usize, usize), &'static str> {
    let atoms = move_str.chars().collect::<Vec<char>>();

    if atoms.len() != 3 {
        return Err(MOVE);
    }

    let x = match atoms[0].to_digit(10) {
        Some(num) => num,
        None => return Err(MOVE),
    };

    if atoms[1] != ',' {
        return Err(MOVE);
    }

    let y = match atoms[2].to_digit(10) {
        Some(num) => num,
        None => return Err(MOVE),
    };

    if x > 2 || y > 2 {
        return Err(MOVE);
    }

    Ok((x as usize, y as usize))
}

enum Player {
    O,
    X,
}

trait TicTacToe {
    fn display(&self);
    fn make_move(&mut self, x: usize, y: usize, player: Player);
    fn check_terminal(&self) -> Option<Player>;
    fn get_score(&self) -> i16;
    fn minimax(&self, depth: u8, maximizing: bool) -> MMaxRes;
}

#[derive(Debug)]
struct MMaxRes {
    to_move: Option<(usize, usize)>,
    score: i16,
}

impl TicTacToe for [[char; 3]; 3] {
    fn display(&self) {
        for (i, row) in self.iter().enumerate() {
            print!(" {} ", row[0]);

            for i in 1..3 {
                print!("| {} ", row[i]);
            }

            if i < 2 {
                println!("\n-----------");
            }
        }
    }

    fn make_move(&mut self, x: usize, y: usize, player: Player) {
        if x > 2 || y > 2 {
            panic!("Move index out of bounds");
        }

        self[y][x] = match player {
            Player::O => 'O',
            Player::X => 'X',
        };
    }

    fn check_terminal(&self) -> Option<Player> {
        for x in 0..3 {
            let rowmid = self[1][x];
            if self[0][x] == rowmid && rowmid == self[2][x] {
                match rowmid {
                    'X' => return Some(Player::X),
                    'O' => return Some(Player::O),
                    _ => (),
                }
            }

            let colmid = self[x][1];
            if self[x][0] == colmid && colmid == self[x][2] {
                match colmid {
                    'X' => return Some(Player::X),
                    'O' => return Some(Player::O),
                    _ => (),
                }
            }
        }

        let diamid = self[1][1];

        if self[0][0] == diamid && diamid == self[2][2] {
            match diamid {
                'X' => return Some(Player::X),
                'O' => return Some(Player::O),
                _ => (),
            }
        }

        if self[0][2] == diamid && diamid == self[2][0] {
            match diamid {
                'X' => return Some(Player::X),
                'O' => return Some(Player::O),
                _ => (),
            }
        }

        None
    }

    fn get_score(&self) -> i16 {
        let mut o = 0_i16;
        let mut x = 0_i16;

        let mut lines = Vec::<(char, char, char)>::with_capacity(8);

        for i in 0..3 {
            lines.push((self[0][i], self[1][i], self[2][i]));
            lines.push((self[i][0], self[i][1], self[i][2]));
        }

        lines.push((self[0][0], self[1][1], self[2][2]));
        lines.push((self[2][0], self[1][1], self[0][2]));

        for line in lines {
            match line {
                ('X', 'X', 'X') => {
                    return -1000;
                }

                ('O', 'O', 'O') => {
                    return 1000;
                }

                ('X', 'X', '_') | ('X', '_', 'X') | ('_', 'X', 'X') => {
                    x += 100;
                }

                ('O', 'O', '_') | ('O', '_', 'O') | ('_', 'O', 'O') => {
                    o += 100;
                }

                ('X', '_', '_') | ('_', 'X', '_') | ('_', '_', 'X') => {
                    x += 10;
                }

                ('O', '_', '_') | ('_', 'O', '_') | ('_', '_', 'O') => {
                    o += 10;
                }

                _ => {}
            }
        }

        o - x
    }

    fn minimax(&self, depth: u8, maximizing: bool) -> MMaxRes {
        if depth == 0 {
            return MMaxRes {
                to_move: None,
                score: self.get_score(),
            };
        }

        match self.check_terminal() {
            Some(p) => match p {
                Player::O => {
                    return MMaxRes {
                        to_move: None,
                        score: 1000,
                    }
                }
                Player::X => {
                    return MMaxRes {
                        to_move: None,
                        score: -1000,
                    }
                }
            },

            _ => {}
        }

        let mut res = MMaxRes {
            to_move: None,
            score: 0,
        };

        for (y, row) in self.iter().enumerate() {
            for (x, sym) in row.iter().enumerate() {
                if sym != &'_' {
                    continue;
                }

                let mut board = self.clone();

                if maximizing {
                    board.make_move(x, y, Player::O);

                    let child_res = board.minimax(depth - 1, !maximizing);

                    if let None = res.to_move {
                        res.score = child_res.score;
                        res.to_move = Some((x, y));
                    }

                    if child_res.score > res.score {
                        res.score = child_res.score;
                        res.to_move = Some((x, y));
                    }
                } else {
                    board.make_move(x, y, Player::X);

                    let child_res = board.minimax(depth - 1, !maximizing);

                    if let None = res.to_move {
                        res.score = child_res.score;
                        res.to_move = Some((x, y));
                    }

                    if child_res.score < res.score {
                        res.score = child_res.score;
                        res.to_move = Some((x, y));
                    }
                }
            }
        }

        res
    }
}
