use crate::{chess::{Bitboard, FILE_CHARS, RANK_CHARS, PIECE_CHARS}, movegen::MoveGenInfo};
use std::{num::ParseIntError, fmt::{self, Formatter, Display}, error::Error};


#[derive(Debug)]
pub enum FENParsingError {
    ParseIntError(ParseIntError),
    FileParseError,
    RankParseError,
    InvalidFen
}

impl Display for FENParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FENParsingError::ParseIntError(e) => write!(f, "{e}"),
            FENParsingError::FileParseError => write!(f, "Invalid file!"),
            FENParsingError::RankParseError => write!(f, "Invalid rank!"),
            FENParsingError::InvalidFen => write!(f, "Inavlid FEN!")
        }
    }
}

impl Error for FENParsingError {}

impl From<ParseIntError> for FENParsingError {
    fn from(e: ParseIntError) -> Self {
        return FENParsingError::ParseIntError(e);
    }
}



pub struct Info {
    pub white: bool,
    pub short_white_rights: bool,
    pub long_white_rights: bool,
    pub short_black_rights: bool,
    pub long_black_rights: bool,
    pub en_passant_sq: Option<usize>,
    pub fifty_count: usize,
    pub fullmove_count: usize,

    pub w_pawns: Bitboard,
    pub b_pawns: Bitboard,
    pub w_knights: Bitboard,
    pub b_knights: Bitboard,
    pub w_bishops: Bitboard,
    pub b_bishops: Bitboard,
    pub w_rooks: Bitboard,
    pub b_rooks: Bitboard,
    pub w_queens: Bitboard,
    pub b_queens: Bitboard,
    pub w_kings: Bitboard,
    pub b_kings: Bitboard,

    pub white_mask: Bitboard,
    pub black_mask: Bitboard,
    pub occupied: Bitboard
}

impl Info {
    fn new() -> Self {
        return Info {
            white: true,
            short_white_rights: true,
            long_white_rights: true,
            short_black_rights: true,
            long_black_rights: true,
            en_passant_sq: None,
            fifty_count: 0,
            fullmove_count: 0,
            w_pawns: Bitboard(0),
            b_pawns: Bitboard(0),
            w_knights: Bitboard(0),
            b_knights: Bitboard(0),
            w_bishops: Bitboard(0),
            b_bishops: Bitboard(0),
            w_rooks: Bitboard(0),
            b_rooks: Bitboard(0),
            w_queens: Bitboard(0),
            b_queens: Bitboard(0),
            w_kings: Bitboard(0),
            b_kings: Bitboard(0), 

            white_mask: Bitboard(0),
            black_mask: Bitboard(0),
            occupied: Bitboard(0)
        }
    }

    pub fn from_fen(fen: &str) -> Result<Self, FENParsingError> {
    
        let mut info: Self = Self::new();

        // split fen along whitespaces and unpack
        let mut blocks: Vec<&str> = fen.split(" ").collect();

        let fullmove_block = blocks.pop().ok_or(FENParsingError::InvalidFen)?;
        let fifty_count_block = blocks.pop().ok_or(FENParsingError::InvalidFen)?;
        let en_passant_block = blocks.pop().ok_or(FENParsingError::InvalidFen)?;
        let castling_block = blocks.pop().ok_or(FENParsingError::InvalidFen)?;
        let player_block = blocks.pop().ok_or(FENParsingError::InvalidFen)?;
        let piece_block = blocks.pop().ok_or(FENParsingError::InvalidFen)?;

        // parse blocks
        {
            // full move count
            info.fullmove_count = fullmove_block.parse::<usize>()?;

            // fifty count
            info.fifty_count = fifty_count_block.parse::<usize>()?;

            // en passant square
            info.en_passant_sq = match en_passant_block {
                "-" => None,
                sq => {
                    let file: usize = match sq.chars().nth(0).ok_or(FENParsingError::FileParseError)? {
                        'A' => 0, 'B' => 1, 'C' => 2, 'D' => 3, 'E' => 4, 'F' => 5, 'G' => 6, 'H' => 7, _ => {panic!("Invalid file!");}
                    };
                    let rank: usize = match sq.chars().nth(1).ok_or(FENParsingError::RankParseError)? {
                        '1' => 0, '2' => 1, '3' => 2, '4' => 3, '5' => 4, '6' => 5, '7' => 6, '8' => 7, _ => {panic!("Invalid rank!");}
                    };
                    Some(file + 8 * rank)
                }
            };

            // castling rights
            {
                info.short_white_rights = false;
                info.long_white_rights = false;
                info.short_black_rights = false;
                info.long_black_rights = false;

                for char in castling_block.chars() {
                    match char {
                        'K' => {info.short_white_rights = true;},
                        'Q' => {info.long_white_rights = true;},
                        'k' => {info.short_black_rights = true;},
                        'q' => {info.long_black_rights = true;},
                        _ => {panic!("Invalid right!");}
                    };
                }
            }

            // player
            info.white = match player_block {
                "w" => true, "b" => false, _ => {panic!("Invalid player!");}
            };

            // place pieces
            {
                let mut file: usize = 0;
                let mut rank: usize = 7;
                for char in piece_block.chars() {

                    let sq: usize = file + 8 * rank;

                    match char {
                        '/' => {rank -= 1; file = 0},
                        '1' => {file += 1;},
                        '2' => {file += 2;},
                        '3' => {file += 3;},
                        '4' => {file += 4;},
                        '5' => {file += 5;},
                        '6' => {file += 6;},
                        '7' => {file += 7;},
                        '8' => {file += 8;},
                        'P' => {info.w_pawns.set_bit(sq); file += 1;},
                        'p' => {info.b_pawns.set_bit(sq); file += 1;},
                        'N' => {info.w_knights.set_bit(sq); file += 1;},
                        'n' => {info.b_knights.set_bit(sq); file += 1;},
                        'B' => {info.w_bishops.set_bit(sq); file += 1;},
                        'b' => {info.b_bishops.set_bit(sq); file += 1;},
                        'R' => {info.w_rooks.set_bit(sq); file += 1;},
                        'r' => {info.b_rooks.set_bit(sq); file += 1;},
                        'Q' => {info.w_queens.set_bit(sq); file += 1;},
                        'q' => {info.b_queens.set_bit(sq); file += 1;},
                        'K' => {info.w_kings.set_bit(sq); file += 1;},
                        'k' => {info.b_kings.set_bit(sq); file += 1;},
                        _ => {panic!("Invalid piece char!");}
                    };
                }
            }
        }
    
        return Ok(info);
    }

    pub fn visualize(self) -> () {

        let mut vis: String = String::from("     a b c d e f g h\n");
        vis.push_str("    -----------------\n");
        for rank in (0..8).rev() {
            vis.push_str(&format!("{}  |", rank+1));
            for file in 0..8 {

                let sq: usize = file + 8 * rank;
                let piece_index: usize = {
                    // ['P', 'p', 'N', 'n', 'B', 'b', 'R', 'r', 'Q', 'q', 'K', 'k', '-']
                    if self.w_pawns.has_entry_at(sq) {0} else
                    if self.b_pawns.has_entry_at(sq) {1} else
                    if self.w_knights.has_entry_at(sq) {2} else
                    if self.b_knights.has_entry_at(sq) {3} else
                    if self.w_bishops.has_entry_at(sq) {4} else
                    if self.b_bishops.has_entry_at(sq) {5} else
                    if self.w_rooks.has_entry_at(sq) {6} else
                    if self.b_rooks.has_entry_at(sq) {7} else
                    if self.w_queens.has_entry_at(sq) {8} else
                    if self.b_queens.has_entry_at(sq) {9} else
                    if self.w_kings.has_entry_at(sq) {10} else
                    if self.b_kings.has_entry_at(sq) {11} else
                    {12}
                };
                let s: &str = &format!(" {}", PIECE_CHARS[piece_index]);

                vis.push_str(s);
            }
            vis.push_str(&format!(" | {}\n", rank+1));
        }
        vis.push_str("    -----------------\n");
        vis.push_str("     a b c d e f g h");
        println!("{}", vis);
    }

    // pub fn get_checkmask(self) -> Bitboard {
    //     let info = MoveGenInfo::<
    //         self.white,
    //         {match self.en_passant_sq {Some(_) => true, None => false}},
    //         self.short_white_rights,
    //         self.long_white_rights,
    //         self.short_black_rights,
    //         self.long_black_rights
    //     > {
    //         w_pawns: self.w_pawns,
    //         b_pawns: self.b_pawns,
    //         w_knights: self.w_knights,
    //         b_knights: self.b_kings,
    //         w_bishops: self.w_bishops,
    //         b_bishops: self.b_bishops,
    //         w_rooks: self.w_rooks,
    //         b_rooks: self.b_rooks,
    //         w_queens: self.w_queens,
    //         b_queens: self.b_queens,
    //         w_kings: self.w_kings,
    //         b_kings: self.b_kings,
    //         white_mask: self.white_mask,
    //         black_mask: self.black_mask,
    //         occupied: self.occupied
    //     };
    //     return Bitboard(0);
    // }
}