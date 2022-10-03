use core::convert::TryInto;
use std::{ops::BitAnd, borrow::BorrowMut};
use crate::min_max::{TwoPlayerGame, Player};
use lazy_static::lazy_static;

pub static PLAYER_CHARS: [char; 3]  = ['w', 'b', '-'];
pub static RANK_CHARS:   [char; 8]  = ['1', '2', '3', '4', '5', '6', '7', '8'];
pub static FILE_CHARS:   [char; 8]  = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
pub static PIECE_CHARS:  [char; 13] = ['P', 'p', 'N', 'n', 'B', 'b', 'R', 'r', 'Q', 'q', 'K', 'k', '-'];  //['.','♙','♘','♗','♖','♕','♔','♟','♞','♝','♜','♛','♚'];

lazy_static! {
    static ref PIECE_KEYS: [[u64; 12]; 64] = {
        let mut piece_keys: [[u64; 12]; 64] = [[0; 12]; 64];

        for sq in 0..64 {
            for piece in 0..12 {
                piece_keys[sq][piece] = rand::random::<u64>();
            }
        }

        return piece_keys;
    };

    static ref SIDE_KEY: u64 = rand::random::<u64>();

    static ref CASTLING_KEYS: [u64; 16] = {
        let mut castling_keys: [u64; 16] = [0; 16];

        for index in 0..16 {
            castling_keys[index] = rand::random::<u64>();
        }
        return castling_keys;
    };

    static ref EN_PASSANT_KEYS: [u64; 64] = {
        let mut en_passant_keys: [u64; 64] = [0; 64];

        for sq in 0..64 {
            en_passant_keys[sq] = rand::random::<u64>();
        }

        return en_passant_keys;
    };
}


type Square = usize;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Piece {
    Wp=0, Bp, Wkn, Bkn, Wb, Bb, Wr, Br, Wq, Bq, Wk, Bk, None=12
}

impl std::convert::TryFrom<usize> for Piece {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Piece::Wp),
            1 => Ok(Piece::Bp),
            2 => Ok(Piece::Wkn),
            3 => Ok(Piece::Bkn),
            4 => Ok(Piece::Wb),
            5 => Ok(Piece::Bb),
            6 => Ok(Piece::Wr),
            7 => Ok(Piece::Br),
            8 => Ok(Piece::Wq),
            9 => Ok(Piece::Bq),
            10 => Ok(Piece::Wk),
            11 => Ok(Piece::Bk),
            12 => Ok(Piece::None),
            _ => Err(())
        }
    }
}

#[derive(Debug)]
pub struct Action {
    pub moving_piece: Piece,
    pub from: Square,
    pub to: Square,
    pub is_capture: bool,
    pub captured_piece: Piece,
    pub is_en_passant: bool,
    pub is_castling: bool,
    pub is_promotion: bool,
    pub promoted_to: Piece,
    pub claim_draw: bool
}

impl Action {
    pub fn to_usize(self) -> usize {
        let mut action: usize = 0;

        action |= self.moving_piece as usize;
        action |= self.from << 4;
        action |= self.to << 12;
        action |= (if self.is_capture {1} else {0}) << 20;
        action |= (self.captured_piece as usize) << 21;
        action |= (if self.is_en_passant {1} else {0}) << 25;
        action |= (if self.is_castling {1} else {0}) << 26;
        action |= (if self.is_promotion {1} else {0}) << 27;
        action |= (self.promoted_to as usize) << 28;
        action |= (if self.claim_draw {1} else {0}) << 29;

        return action
    }

    pub fn from_usize(action: usize) -> Self {
        return Self {
            moving_piece: (action & 0xF).try_into().unwrap(),
            from: (action & 0xFF0) >> 4,
            to: (action & 0xFF000) >> 12 ,
            is_capture: (action & 0x100000) != 0,
            captured_piece: ((action & 0x1E00000) >> 21).try_into().unwrap(),
            is_en_passant: (action & 0x2000000) != 0,
            is_castling: (action & 0x4000000) != 0,
            is_promotion: (action & 0x8000000) != 0,
            promoted_to: ((action & 0xF0000000) >> 28).try_into().unwrap(),
            claim_draw: (action & 0x100000000) != 0
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);

impl Bitboard {

    #[inline(always)]
    pub const fn lsh(self, shift: usize) -> Bitboard {
        Bitboard(self.0 << shift)
    }

    #[inline(always)]
    pub const fn rsh(self, shift: usize) -> Bitboard {
        Bitboard(self.0 << shift)
    }

    #[inline(always)]
    pub const fn and(self, rhs: Bitboard) -> Bitboard {
        Bitboard(self.0 & rhs.0)
    }

    #[inline(always)]
    pub const fn or(self, rhs: Bitboard) -> Bitboard {
        Bitboard(self.0 | rhs.0)
    }

    pub fn set_bit(&mut self, index: usize) -> () {
        //set a bit at the given square of the bitboard
        self.0 |= 1u64 << index;
    }

    pub fn clear_bit(&mut self, index: usize) -> () {
        //clear a bit at the given square of the bitboard
        self.0 &= !(1u64 << index);
    }

    #[inline(always)]
    pub const fn tzcnt(self) -> usize {
        self.0.trailing_zeros() as usize
    }

    #[inline(always)]
    pub const fn blsr(self) -> Bitboard {
        Bitboard(self.0 & self.0.wrapping_sub(1))
    }
    
    #[inline(always)]
    pub const fn has_entry_at(&self, index: usize) -> bool {
        return (self.0 & (1 << index)) != 0u64;
    }

    #[inline(always)]
    pub const fn has_bits(self) -> bool {
        self.0 != 0
    }

    #[inline(always)]
    pub const fn neg(self) -> Bitboard {
        Bitboard(!self.0)
    }

    #[inline(always)]
    pub const fn number_of_set_bits(self) -> usize {
        self.0.count_ones() as usize
    }

    pub const fn const_pext(self, mask: Bitboard) -> usize {
        let mut k = 0;
        let mut m = 0;
        let mut out = 0;
        while m < 64 {
            if (mask.0 & (1 << m)) != 0 {
                if (self.0 & (1 << m)) != 0 {
                    out |= 1 << k;
                } else {
                    out &= !(1 << k);
                }
                k += 1;
            }
            m += 1;
        }
        out
    }

    #[inline(always)]
    pub fn unsafe_pext(self, mask: Bitboard) -> usize {
        unsafe {
            core::arch::x86_64::_pext_u64(self.0, mask.0) as usize
        }
    }

    pub fn visualize(&self) -> () {
        for rank in (0..8).rev() {
            for file in 0..8 {
                if (1u64 << (file + 8 * rank) & self.0) != 0u64 {
                    print!("1 ");
                } else {
                    print!("- ");
                }
            }
            print!("\n");
        }
        print!("\n");
    }

}

impl std::ops::BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

#[derive(Clone, Copy)]
struct ZobristHash {
    pub key: u64
}

impl ZobristHash {
    pub fn hash_piece(&mut self, piece: Piece, sq: Square) -> () {
        // hashes (piece, square) key into board key
        self.key ^= PIECE_KEYS[sq][piece as usize];
    }
    pub fn hash_cast(&mut self, castling_rights: u8) {
        // hashes castling key in and out of board key
        self.key ^= CASTLING_KEYS[castling_rights as usize];
    }
    pub fn hash_side(&mut self) {
        // hashes side key in and out of board key
        self.key ^= *SIDE_KEY;
    }
    pub fn hash_en_passant(&mut self, sq: Square) {
        // hashes en passant square key into an out of board key
        self.key ^= EN_PASSANT_KEYS[sq];
    }
}

pub struct Chess {
    current_player: Player,

    w_pawns: Bitboard,
    b_pawns: Bitboard,
    w_knights: Bitboard,
    b_knights: Bitboard,
    w_bishops: Bitboard,
    b_bishops: Bitboard,
    w_rooks: Bitboard,
    b_rooks: Bitboard,
    w_queens: Bitboard,
    b_queens: Bitboard,
    w_kings: Bitboard,
    b_kings: Bitboard,

    white_mask: Bitboard,
    black_mask: Bitboard,

    castle_permissions: u8,
    en_passant_square: Option<Square>,

    zobrist_hash: ZobristHash,
    history_of_hashes: Vec<ZobristHash>,
    hundred_count: usize

}

impl Chess {
    pub fn new() -> Self {

        // create hash
        let mut hash: ZobristHash = ZobristHash{key: 064};

        // hash pieces in
        for (piece, sq) in std::iter::repeat(Piece::Wp).take(8).zip(8..16) {
            hash.hash_piece(piece, sq);
        }
        for (piece, sq) in [
            (Piece::Wkn, 1), (Piece::Wkn, 6),
            (Piece::Wb, 2), (Piece::Wb, 5),
            (Piece::Wr, 0), (Piece::Wr, 7),
            (Piece::Wq, 3), (Piece::Wk, 4)
        ].iter() {
            hash.hash_piece(*piece, *sq);
        }
        for (piece, sq) in std::iter::repeat(Piece::Bp).take(8).zip(48..56) {
            hash.hash_piece(piece, sq);
        }
        for (piece, sq) in [
                (Piece::Bkn, 57), (Piece::Bkn, 62),
                (Piece::Bb, 58), (Piece::Bb, 61),
                (Piece::Br, 56), (Piece::Br, 63),
                (Piece::Bq, 59), (Piece::Bk, 60)
            ].iter() {
            hash.hash_piece(*piece, *sq);
        }

        // hash castling rights in
        for right in [1, 2, 4, 8].iter() {
            hash.hash_cast(*right)
        }

        // hash piece key in as it is whites turn
        hash.hash_side();

        return Self {
            current_player: Player::White,

            w_pawns: Bitboard(0xFF00),
            b_pawns: Bitboard(0xFF000000000000),
            w_knights: Bitboard(0x42),
            b_knights: Bitboard(0x4200000000000000),
            w_bishops: Bitboard(0x24),
            b_bishops: Bitboard(0x2400000000000000),
            w_rooks: Bitboard(0x81),
            b_rooks: Bitboard(0x8100000000000000),
            w_queens: Bitboard(0x8),
            b_queens: Bitboard(0x800000000000000),
            w_kings: Bitboard(0x10),
            b_kings: Bitboard(0x1000000000000000),

            white_mask: Bitboard(0xFFFF),
            black_mask: Bitboard(0xFFFF000000000000),

            castle_permissions: 1 + 2 + 4 + 8,
            en_passant_square: Option::None,

            zobrist_hash: hash,
            history_of_hashes: Vec::new(),
            hundred_count: 0
        }
    }

    pub fn visualize(&self) -> () {

        let mut vis: String = String::from("\n");
        vis.push_str(
            &format!(
                "     {} {} {} {} {} {} {} {}\n",
                FILE_CHARS[0], FILE_CHARS[1], FILE_CHARS[2], FILE_CHARS[3],
                FILE_CHARS[4], FILE_CHARS[5], FILE_CHARS[6], FILE_CHARS[7]
            )
        );
        vis.push_str("    ________________\n");

        let bbs = [
            self.w_pawns, self.b_pawns,
            self.w_knights, self.b_knights,
            self.w_bishops, self.b_bishops,
            self.w_rooks, self.b_rooks,
            self.w_queens, self.b_queens,
            self.w_kings, self.b_kings
        ];
        let pieces = [
            Piece::Wp, Piece::Bp,
            Piece::Wkn, Piece::Bkn,
            Piece::Wb, Piece::Bb,
            Piece::Wr, Piece::Br,
            Piece::Wq, Piece::Bq,
            Piece::Wk, Piece::Bk
        ];


        for rank in (0..8).rev() {

            vis.push_str(&format!(" {} |", RANK_CHARS[rank]));

            for file in 0..8 {

                let sq: usize = file + 8 * rank;

                let mut piece_found: bool = false;
                for (bb, &piece) in bbs.iter().zip(pieces.iter()) {
                    if bb.has_entry_at(sq) {
                        let piece_char: char = PIECE_CHARS[piece as usize];
                        vis.push_str(&format!(" {}", piece_char));
                        piece_found = true;
                        break;
                    }
                }

                if !piece_found {
                    vis.push_str(&format!(" {}", PIECE_CHARS[12]));
                }
            }

            vis.push('\n');
        }

        vis.push_str(
            &format!(
                "\nplayer to move: {:?},\nen passant on: {},\nZobrist-hash: {:#x}",
                self.current_player,
                match self.en_passant_square {
                    Option::None => String::from("-"),
                    Option::Some(idx) => [FILE_CHARS[idx % 8], RANK_CHARS[idx / 8]]
                        .iter().cloned().collect()
                },
                self.zobrist_hash.key
            )
        );

        println!("{}", vis);
    }

    fn add_piece(&mut self, piece: Piece, sq: Square) -> () {

        // choose right bitboards to add to
        let (bb, mask): (&mut Bitboard, &mut Bitboard) = match piece {
            Piece::Wp => (&mut self.w_pawns, &mut self.white_mask),
            Piece::Bp => (&mut self.b_pawns, &mut self.black_mask),
            Piece::Wkn => (&mut self.w_knights, &mut self.white_mask),
            Piece::Bkn => (&mut self.b_knights, &mut self.black_mask),
            Piece::Wb => (&mut self.w_bishops, &mut self.white_mask),
            Piece::Bb => (&mut self.b_bishops, &mut self.black_mask),
            Piece::Wr => (&mut self.w_rooks, &mut self.white_mask),
            Piece::Br => (&mut self.b_rooks, &mut self.black_mask),
            Piece::Wq => (&mut self.w_queens, &mut self.white_mask),
            Piece::Bq => (&mut self.b_queens, &mut self.black_mask),
            Piece::Wk => (&mut self.w_knights, &mut self.white_mask),
            Piece::Bk => (&mut self.b_knights, &mut self.black_mask),
            Piece::None => panic!("Can't remove from empty square!")
        };

        // hash piece in and add bits to bitboard(s)
        self.zobrist_hash.hash_piece(piece, sq);
        bb.set_bit(sq);
        mask.set_bit(sq);
    }

    fn clear_piece(&mut self, piece: Piece, sq: Square) -> () {

        // choose right bitboards to clear from
        let (bb, mask): (&mut Bitboard, &mut Bitboard) = match piece {
            Piece::Wp => (&mut self.w_pawns, &mut self.white_mask),
            Piece::Bp => (&mut self.b_pawns, &mut self.black_mask),
            Piece::Wkn => (&mut self.w_knights, &mut self.white_mask),
            Piece::Bkn => (&mut self.b_knights, &mut self.black_mask),
            Piece::Wb => (&mut self.w_bishops, &mut self.white_mask),
            Piece::Bb => (&mut self.b_bishops, &mut self.black_mask),
            Piece::Wr => (&mut self.w_rooks, &mut self.white_mask),
            Piece::Br => (&mut self.b_rooks, &mut self.black_mask),
            Piece::Wq => (&mut self.w_queens, &mut self.white_mask),
            Piece::Bq => (&mut self.b_queens, &mut self.black_mask),
            Piece::Wk => (&mut self.w_knights, &mut self.white_mask),
            Piece::Bk => (&mut self.b_knights, &mut self.black_mask),
            Piece::None => panic!("Can't remove from empty square!")
        };

        // hash piece out and clear bits to bitboard(s)
        self.zobrist_hash.hash_piece(piece, sq);
        bb.clear_bit(sq);
        mask.clear_bit(sq);
    }

    fn move_piece(&mut self, piece: Piece, sq1: Square, sq2: Square) -> () {

        // choose right bitboards to modify
        let (bb, mask): (&mut Bitboard, &mut Bitboard) = match piece {
            Piece::Wp => (&mut self.w_pawns, &mut self.white_mask),
            Piece::Bp => (&mut self.b_pawns, &mut self.black_mask),
            Piece::Wkn => (&mut self.w_knights, &mut self.white_mask),
            Piece::Bkn => (&mut self.b_knights, &mut self.black_mask),
            Piece::Wb => (&mut self.w_bishops, &mut self.white_mask),
            Piece::Bb => (&mut self.b_bishops, &mut self.black_mask),
            Piece::Wr => (&mut self.w_rooks, &mut self.white_mask),
            Piece::Br => (&mut self.b_rooks, &mut self.black_mask),
            Piece::Wq => (&mut self.w_queens, &mut self.white_mask),
            Piece::Bq => (&mut self.b_queens, &mut self.black_mask),
            Piece::Wk => (&mut self.w_knights, &mut self.white_mask),
            Piece::Bk => (&mut self.b_knights, &mut self.black_mask),
            Piece::None => panic!("Can't remove from empty square!")
        };

        // hash piece out and back in, then clear and set bits to bitboard(s)
        self.zobrist_hash.hash_piece(piece, sq1);
        self.zobrist_hash.hash_piece(piece, sq2);
        bb.clear_bit(sq1);
        mask.clear_bit(sq1);
        bb.set_bit(sq2);
        mask.set_bit(sq2);
    }
}

impl TwoPlayerGame for Chess {
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

        // push current key to history
        self.history_of_hashes.push(self.zobrist_hash);

        // get current player, then swap, then hash
        let player: Player = self.current_player;
        self.current_player = self.current_player.swap();
        self.zobrist_hash.hash_side();

        // parse action
        let action: Action = Action::from_usize(action);

        // handle castling
        if action.is_castling {
            // match "to" square to know which kind of castling we deal with, then:
            //    1. move king, 2. move rook, 3. update castling rights (hashing is handled later)
            match action.to {
                6 => {
                    self.move_piece(Piece::Wk, action.from, action.to);
                    self.move_piece(Piece::Wr, 7, 5);
                    self.castle_permissions &= !1;
                },
                2 => {
                    self.move_piece(Piece::Wk, action.from, action.to);
                    self.move_piece(Piece::Wr, 0, 3);
                    self.castle_permissions &= !2;
                },
                62 => {
                    self.move_piece(Piece::Bk, action.from, action.to);
                    self.move_piece(Piece::Br, 63, 61);
                    self.castle_permissions &= !4;
                },
                58 => {
                    self.move_piece(Piece::Bk, action.from, action.to);
                    self.move_piece(Piece::Br, 56, 59);
                    self.castle_permissions &= !8;
                },
                _ => panic!("Invalid castling!")
            }
        }

        // update castle permissions:
        //    1. remove permissions, 2. hash out permission
        match action.from {
            4 => {
                self.castle_permissions &= !(1 | 2);
                self.zobrist_hash.hash_cast(1);
                self.zobrist_hash.hash_cast(2);
            },
            60 => {
                self.castle_permissions &= !(4 | 8);
                self.zobrist_hash.hash_cast(4);
                self.zobrist_hash.hash_cast(8);
            },
            0 => {
                self.castle_permissions &= !2;
                self.zobrist_hash.hash_cast(2);
            },
            7 => {
                self.castle_permissions &= !1;
                self.zobrist_hash.hash_cast(1);
            },
            56 => {
                self.castle_permissions &= !8;
                self.zobrist_hash.hash_cast(8);
            },
            63 => {
                self.castle_permissions &= !4;
                self.zobrist_hash.hash_cast(4);
            },
            _ => {}
        }

        // handle en passant square
        if action.is_en_passant {
            // reset en passant square
            self.en_passant_square = Option::None;

            // hash it out
            match player {
                Player::White => self.zobrist_hash.hash_en_passant(action.to - 8),
                Player::Black => self.zobrist_hash.hash_en_passant(action.to + 8),
                Player::None => panic!("")
            }
        } else {
            // check for pawn start
            let cond1: bool = (action.moving_piece == Piece::Wp) & (action.from + 16 == action.to);
            let cond2: bool = (action.moving_piece == Piece::Bp) & (action.to + 16 == action.from);
            if cond1 {
                let sq: Square = action.from + 8;
                self.en_passant_square = Option::Some(sq);  // set square
                self.zobrist_hash.hash_en_passant(sq);  // hash it in
            } else if cond2 {
                let sq: Square = action.to + 8;
                self.en_passant_square = Option::Some(sq);  // set square
                self.zobrist_hash.hash_en_passant(sq);  // hash it in
            } else {
                self.en_passant_square = Option::None;
            }
        }

        // handle 100 count
        if action.is_capture
            || (action.moving_piece == Piece::Wp)
            || (action.moving_piece == Piece::Bp)
        {
            self.hundred_count = 0;
        } else {
            self.hundred_count += 1;
        }

        // handle capture
        if action.is_capture {

            if action.is_en_passant {
                // clear taken piece
                match player {
                    Player::White => self.clear_piece(Piece::Bp, action.to - 8),
                    Player::Black => self.clear_piece(Piece::Wp, action.to + 8),
                    Player::None => panic!("")
                }
            } else {
                self.clear_piece(action.captured_piece, action.to);
            }
        }

        // move piece
        self.move_piece(action.moving_piece, action.from, action.to);

        // handle promotion
        if action.is_promotion {
            self.clear_piece(action.moving_piece, action.to);
            self.add_piece(action.promoted_to, action.to);
        }
    }

    fn undo(&mut self) -> () {
        todo!()
    }
}

// Example game

// let mut game = chess::Chess::new();
// game.visualize();

// let action: usize = Action::to_usize(Action{
//     moving_piece: Piece::Wp,
//     from: 11,
//     to: 27,
//     is_capture: false,
//     captured_piece: Piece::None,
//     is_en_passant: false,
//     is_castling: false,
//     is_promotion: false,
//     promoted_to: Piece::None,
//     claim_draw: false
// });

// game.step(action);

// game.visualize();

// let action: usize = Action::to_usize(Action{
//     moving_piece: Piece::Bp,
//     from: 51,
//     to: 35,
//     is_capture: false,
//     captured_piece: Piece::None,
//     is_en_passant: false,
//     is_castling: false,
//     is_promotion: false,
//     promoted_to: Piece::None,
//     claim_draw: false
// });

// game.step(action);

// game.visualize();let action: usize = Action::to_usize(Action{
//     moving_piece: Piece::Wkn,
//     from: 1,
//     to: 18,
//     is_capture: false,
//     captured_piece: Piece::None,
//     is_en_passant: false,
//     is_castling: false,
//     is_promotion: false,
//     promoted_to: Piece::None,
//     claim_draw: false
// });

// game.step(action);

// game.visualize();let action: usize = Action::to_usize(Action{
//     moving_piece: Piece::Bp,
//     from: 52,
//     to: 44,
//     is_capture: false,
//     captured_piece: Piece::None,
//     is_en_passant: false,
//     is_castling: false,
//     is_promotion: false,
//     promoted_to: Piece::None,
//     claim_draw: false
// });

// game.step(action);

// game.visualize();let action: usize = Action::to_usize(Action{
//     moving_piece: Piece::Wkn,
//     from: 18,
//     to: 35,
//     is_capture: true,
//     captured_piece: Piece::Bp,
//     is_en_passant: false,
//     is_castling: false,
//     is_promotion: false,
//     promoted_to: Piece::None,
//     claim_draw: false
// });

// game.step(action);

// game.visualize();let action: usize = Action::to_usize(Action{
//     moving_piece: Piece::Bp,
//     from: 44,
//     to: 35,
//     is_capture: true,
//     captured_piece: Piece::Wkn,
//     is_en_passant: false,
//     is_castling: false,
//     is_promotion: false,
//     promoted_to: Piece::None,
//     claim_draw: false
// });

// game.step(action);

// game.visualize();