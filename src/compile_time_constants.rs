
use std::collections::btree_map::OccupiedEntry;

use crate::chess::Bitboard;

// toggleable assert
pub const USE_RUNTIME_ASSERT: bool = true;
macro_rules! compiletime_toggleable_assert {
    ( $( $x:expr ),* ) => {
        {
            $(
                if crate::compile_time_constants::USE_RUNTIME_ASSERT {
                    if !($x) {
                        panic!("Assert failed!");
                    }
                }
            )*
        }
    };
}

macro_rules! bitloop {
    ( $bb:expr , $var:ident => $body:expr ) => {
        let mut bb = $bb;
        while bb.has_bits() {
            let $var: usize = bb.tzcnt();

            $body

            bb = bb.blsr();
        }
    }
}

pub(crate) use compiletime_toggleable_assert;
pub(crate) use bitloop;

pub const WHITE_PAWN_KING_ATTACKS: [Bitboard; 64] = {

    let mut pawn_mask: [Bitboard; 64] = [Bitboard(0); 64];

    let mut sq: usize = 0;
    while sq < 64 {

        let (x, y) = (sq % 8, sq / 8);

        let mut mask: u64 = 0;
        if x > 0 && y > 1 {mask |= 1 << (sq - 9)}
        if x < 7 && y > 1 {mask |= 1 << (sq - 7)}

        pawn_mask[sq] = Bitboard(mask);

        sq += 1;
    }

    pawn_mask
};

pub const BLACK_PAWN_KING_ATTACKS: [Bitboard; 64] = {

    let mut pawn_mask: [Bitboard; 64] = [Bitboard(0); 64];

    let mut sq: usize = 0;
    while sq < 64 {

        let (x, y) = (sq % 8, sq / 8);

        let mut mask: u64 = 0;
        if x > 0 && y < 7 {mask |= 1 << (sq + 7)}
        if x < 7 && y < 7 {mask |= 1 << (sq + 9)}

        pawn_mask[sq] = Bitboard(mask);

        sq += 1;
    }

    pawn_mask
};

pub const KNIGHT_MASK: [Bitboard; 64] = {

    let mut knight_masks: [Bitboard; 64] = [Bitboard(0); 64];

    // loop through all squares
    let mut sq: usize = 0;
    while sq < 64 {

        let mut pattern: u64 = 0u64;

        let (x, y): (usize, usize) = (sq % 8, sq / 8);

        // at pattern (attention to walking over edges of board)
        if (x >= 1) & (y >= 2) {pattern |= 1 << (sq - 16 - 1)};
        if (x <= 6) & (y >= 2) {pattern |= 1 << (sq - 16 + 1)};
        if (x >= 2) & (y >= 1) {pattern |= 1 << (sq - 8 - 2)};
        if (x <= 5) & (y >= 1) {pattern |= 1 << (sq - 8 + 2)};
        if (x >= 2) & (y <= 6) {pattern |= 1 << (sq + 8 - 2)};
        if (x <= 5) & (y <= 6) {pattern |= 1 << (sq + 8 + 2)};
        if (x >= 1) & (y <= 5) {pattern |= 1 << (sq + 16 - 1)};
        if (x <= 6) & (y <= 5) {pattern |= 1 << (sq + 16 + 1)};

        knight_masks[sq] = Bitboard(pattern);

        sq += 1;
    }
    knight_masks
};

pub const PLUS_MASK: [Bitboard; 64] = {

    let mut plus_masks: [Bitboard; 64] = [Bitboard(0); 64];

    let horizontal_mask: u64 = 0xFF;
    let vertical_mask: u64 = 0x0101010101010101;

    // loop through all squares
    let mut sq: usize = 0;
    while sq < 64 {

        let mut pattern: u64 = 0;

        let (x, y): (usize, usize) = (sq % 8, sq / 8);

        pattern |= horizontal_mask << 8 * y;
        pattern |= vertical_mask << x;

        plus_masks[sq] = Bitboard(pattern);

        sq += 1;
    }

    plus_masks
};

pub const X_MASK: [Bitboard; 64] = {

    let mut x_masks: [Bitboard; 64] = [Bitboard(0); 64];

    let mut sq: usize = 0;
    while sq < 64 {

        let mut pattern: u64 = 0;

        let (x, y): (usize, usize) = (sq % 8, sq / 8);

        // ur ray
        let mut dx: usize = 0;
        let mut dy: usize = 0;
        while (x + dx < 8) & (y + dy < 8) {
            pattern |= 1 << (x + dx + 8 * (y + dy));
            dx += 1;
            dy += 1
        }

        // dr ray
        let mut dx: usize = 0;
        let mut dy: usize = 0;
        while (x + dx < 8) & (dy <= y) {
            pattern |= 1 << (x + dx + 8 * (y - dy));
            dx += 1;
            dy += 1
        }

        // dl ray
        let mut dx: usize = 0;
        let mut dy: usize = 0;
        while (dx <= x) & (dy <= y) {
            pattern |= 1 << (x - dx + 8 * (y - dy));
            dx += 1;
            dy += 1
        }

        // ul ray
        let mut dx: usize = 0;
        let mut dy: usize = 0;
        while (dx <= x) & (y + dy < 8) {
            pattern |= 1 << (x - dx + 8 * (y + dy));
            dx += 1;
            dy += 1
        }

        x_masks[sq] = Bitboard(pattern);

        sq += 1;
    }
    
    x_masks
}; 


// // the x_ray mask for each pair of (king square, slider square, slider type)
// const X_RAY_MASK: [[[Bitboard; 64]; 64]; 1+1+1] = {
//     todo!();
// };