use core::convert::TryInto;
use crate::min_max::{TwoPlayerGame, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Figure {
    None, WPawn, WKing, BPawn, BKing
}

pub struct Checkers {
    current_player: Player,
    board: [[Figure; 8]; 8],
    
    // for undo
    jumps: Vec<((usize, usize), (usize, usize))>,
    jumped_piece: Vec<Figure>,
    jumps_per_move: Vec<usize>
}

impl Default for Checkers {
    fn default() -> Self {
        let mut game: Self = Self {
            current_player: Player::Black,
            board: [[Figure::None; 8]; 8],
            
            jumps: Vec::new(),
            jumped_piece: Vec::new(),
            jumps_per_move: Vec::new(),
        };

        let black_coords: [(usize, usize); 12] = [
            (1, 7), (3, 7), (5, 7), (7, 7),
            (0, 6), (2, 6), (4, 6), (6, 6),
            (1, 5), (3, 5), (5, 5), (7, 5)
        ];
        let white_coords: [(usize, usize); 12] = [
            (0, 0), (2, 0), (4, 0), (6, 0),
            (1, 1), (3, 1), (5, 1), (7, 1),
            (0, 2), (2, 2), (4, 2), (6, 2),
        ];

        for (x, y) in black_coords.iter() {
            game.board[*x][*y] = Figure::BPawn;
        }

        for (x, y) in white_coords.iter() {
            game.board[*x][*y] = Figure::WPawn;
        }

        return game;
    }
}

impl Checkers {
    pub fn visualize(&self) -> () {
        let mut vis: String = String::from("     a b c d e f g h\n");
        vis.push_str("    ________________\n");
        for y in (0..8).rev() {
            vis.push_str(&format!("{}  |", y+1));
            for x in 0..8 {
                let s: &str = match self.board[x][y] {
                    Figure::None  => " .",
                    Figure::BPawn => " x",
                    Figure::BKing => " X",
                    Figure::WPawn => " o",
                    Figure::WKing => " O"
                };
                vis.push_str(s);
            }
            vis.push_str(&format!(" | {}\n", y));
        }
        vis.push_str("    ________________\n");
        vis.push_str("     0 1 2 3 4 5 6 7");
        println!("{}", vis);
    }

    fn parse_usize_action(mut action: usize) -> (Vec<(usize, usize)>, usize) {
        // decompose usize into jumped squares,
        // last 4 bits give information how many squares were jumped
        // the squares are consecutively listed as u5s

        // get number of jumps
        let n_squares: usize = action >> 60;

        // extract squares
        let mut squares: Vec<usize> = Vec::new();
        for _ in 0..n_squares {
            squares.push(action % 0x20);
            action = action >> 5;
        }

        // convert to coordinate representation
        let mut coords: Vec<(usize, usize)> = Vec::new();
        for square in squares {
            let x: usize = match square % 8 {
                0 => 0, 4 => 1, 1 => 2, 5 => 3, 2 => 4, 6 => 5, 3 => 6, 7 => 7, _ => panic!("")
            };
            let y: usize = square / 4;
            println!("{}, {}", x, y);
            coords.push((x, y));
        }
        return (coords, n_squares);
    }
}

impl TwoPlayerGame for Checkers {
    fn current_player(&self) -> Player {
        return self.current_player;
    }

    fn eval(&self) -> (f32, bool) {
        todo!()
    }

    fn get_legal_actions(&self) -> Vec<usize> {
        todo!()
    }

    fn step(&mut self, action: usize) -> () {

        // todo!("Handle promotion!");

        // parse action
        let (coords, n_squares) = Checkers::parse_usize_action(action);

        // register how many jumps occur
        self.jumps_per_move.push(n_squares - 1);

        fn add_diff(a: usize, b: usize) -> usize {
                let mut a_int: i32 = a.try_into().unwrap();
                let b_int: i32 = b.try_into().unwrap();

                a_int += (b_int - a_int) / 2;

                return a_int.try_into().unwrap();
            }

        // move piece, remove jumped pieces register to undo-history
        for (start_coords, end_coords) in coords.iter().zip(coords.iter().skip(1)) {

            let man_in_the_middle: (usize, usize) = (
                add_diff(start_coords.0, end_coords.0),
                add_diff(start_coords.1, end_coords.1)
            );

            // handle captures
            if man_in_the_middle != *start_coords {
                let (x, y) = man_in_the_middle;
                self.jumps.push((*start_coords, *end_coords));  // push jump
                self.jumped_piece.push(self.board[x][y]);  // push jumped piece
                self.board[x][y] = Figure::None;  // remove jumped piece
            }

            // move jumping piece
            println!("{:?}", self.board[start_coords.0][start_coords.1]);
            self.board[end_coords.0][end_coords.1] = self.board[start_coords.0][start_coords.1];
            self.board[start_coords.0][start_coords.1] = Figure::None;
        }
    }

    fn undo(&mut self) -> () {
        todo!()
    }
}