use core::convert::TryInto;
use core::default::Default;
use crate::min_max::{TwoPlayerGame, Player};

const X_SHAPE: usize = 8;
const Y_SHAPE: usize = 5;

type Position = (usize, usize);

type BoardArray<T> = [[T; Y_SHAPE]; X_SHAPE];

impl Player {
    pub fn swap(self) -> Player {
        return match self {
            Player::White => Player::Black,
            Player::Black => Player::White,
            Player::None  => panic!("Invalid player!")
        };
    }
}

#[derive(Clone)]
pub struct Connect4 {
    current_player: Player,
    board: BoardArray<Player>,
    history: Vec<Position>,
}

impl Default for Connect4 {
    fn default() -> Self {
        return Connect4{
            current_player: Player::White,
            board: [[Player::None; Y_SHAPE]; X_SHAPE],
            history: Vec::with_capacity(X_SHAPE * Y_SHAPE),
        }
    }
}

impl Connect4 {

    fn evaluate(&self) -> Player {

        // todo!("Make more efficient by only checking last dropped square!");


        // check horizontal
        for y in 0..Y_SHAPE {
            // variables to keep track of how many squares of a color have been seen in a row
            let mut black_counter: usize = 0;
            let mut white_counter: usize = 0;

            // loop through all squares in row y
            for x in 0..X_SHAPE {
                match self.board[x][y] {
                    Player::Black => {black_counter += 1; white_counter = 0;},
                    Player::White => {white_counter += 1; black_counter = 0;},
                    Player::None  => {black_counter = 0; white_counter = 0;}
                }

                if black_counter >= 4 {
                    return Player::Black
                } else if white_counter >= 4 {
                    return Player::White
                }

            }
        }

        // check vertical
        for x in 0..X_SHAPE {
            // variables to keep track of how many squares of a color have been seen in a row
            let mut black_counter: usize = 0;
            let mut white_counter: usize = 0;

            // loop through all squares in column y
            for y in 0..Y_SHAPE {
                match self.board[x][y] {
                    Player::Black => {
                        black_counter += 1;
                        white_counter = 0;
                    },
                    Player::White => {
                        white_counter += 1;
                        black_counter = 0;
                    },
                    Player::None => {
                        black_counter = 0;
                        white_counter = 0;
                    }
                }

                if black_counter >= 4 {
                    return Player::Black
                } else if white_counter >= 4 {
                    return Player::White
                }
            }
        }

        // check diagonal "u-r"
        let x_iter = std::iter::repeat(0).take(Y_SHAPE-1).chain(0..X_SHAPE);
        let y_iter = (0..Y_SHAPE).rev().chain( std::iter::repeat(0).take(X_SHAPE) );
        let x_y_iter = x_iter.zip(y_iter);

        for (x, y) in x_y_iter {
            // variables to keep track of how many squares of a color have been seen in a row
            let mut black_counter: usize = 0;
            let mut white_counter: usize = 0;

            let mut xd = x;
            let mut yd = y;
            let (dx, dy) = (1, 1);

            while xd + dx < X_SHAPE && yd + dy < Y_SHAPE {

                match self.board[xd][yd] {
                    Player::Black => {black_counter += 1; white_counter = 0;},
                    Player::White => {white_counter += 1; black_counter = 0;},
                    Player::None  => {black_counter = 0; white_counter = 0;}
                }

                if black_counter >= 4 {
                    return Player::Black
                } else if white_counter >= 4 {
                    return Player::White
                }

                xd = xd + dx;
                yd = yd + dy;
            }
        }

        // check diagonal "u-l"
        let x_iter = std::iter::repeat(X_SHAPE-1).take(Y_SHAPE-1).chain((0..X_SHAPE).rev());
        let y_iter = (0..Y_SHAPE).rev().chain( std::iter::repeat(0).take(X_SHAPE) );
        let x_y_iter = x_iter.zip(y_iter);

        for (x, y) in x_y_iter {

            // variables to keep track of how many squares of a color have been seen in a row
            let mut black_counter: usize = 0;
            let mut white_counter: usize = 0;

            let mut xd: i32 = x.try_into().unwrap();
            let mut yd: i32 = y.try_into().unwrap();
            let (dx, dy) = (-1, 1);

            while 0 <= xd + dx && yd + dy < Y_SHAPE.try_into().unwrap() {

                let x_idx: usize = xd.try_into().unwrap();
                let y_idx: usize = yd.try_into().unwrap();
                match self.board[x_idx][y_idx] {
                    Player::Black => {black_counter += 1; white_counter = 0;},
                    Player::White => {white_counter += 1; black_counter = 0;},
                    Player::None  => {black_counter = 0; white_counter = 0;}
                }

                if black_counter >= 4 {
                    return Player::Black
                } else if white_counter >= 4 {
                    return Player::White
                }

                xd = xd + dx;
                yd = yd + dy;
            }
        }

        return Player::None;
    }

    pub fn visualize(&self) -> () {

        let mut vis: String = "".to_owned();

        for y in (0..Y_SHAPE).rev() {
            vis.push_str("| ");
            for x in 0..X_SHAPE {

                vis.push(
                    match self.board[x][y] {
                        Player::White => 'O',
                        Player::Black => 'X',
                        Player::None  => ' '
                    }
                );

                vis.push_str(" | ");
            }
            vis.push('\n');
        }

        println!("{}", vis);
        println!("{:?} to move!", self.current_player);
    }
}

impl TwoPlayerGame for Connect4 {
    fn current_player(&self) -> Player {
        return self.current_player;
    }
    
    fn eval(&self) -> (f32, bool) {
        let player: Player = self.evaluate();

        return match player {
            Player::Black => (-1f32, true),
            Player::White => (1f32, true),
            Player::None  => (0f32, false)
        }
    }

    fn get_legal_actions(&self) -> Vec<usize> {

        let mut legal_move_mask: [bool; X_SHAPE] = [false; X_SHAPE];

        if self.evaluate() != Player::None {
            return Vec::with_capacity(0)
        }

        for x in 0..X_SHAPE {
            legal_move_mask[x] = (self.board[x])[Y_SHAPE - 1] == Player::None
        }

        return (0..X_SHAPE).filter(|&action| legal_move_mask[action]).collect();
    }

    fn step(&mut self, x: usize) -> () {

        // find row of dropped stone
        let y: usize = self.board[x]
            .iter()
            .position(|&r| r == Player::None)
            .unwrap();

        // place new stone
        self.board[x][y] = self.current_player;

        // swap player
        self.current_player = self.current_player.swap();

        // append new move to history
        self.history.push((x, y));
    }

    fn undo(&mut self) -> () {

        // pop last move from history
        let (x, y): Position = self.history.pop().unwrap();

        // remove stone from board and swap player
        self.board[x][y] = Player::None;
        self.current_player = self.current_player.swap();
    }
}