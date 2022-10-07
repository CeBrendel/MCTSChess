
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

const fn min(a: usize, b: usize) -> usize {
    if a > b {b} else {a}
}

// will change to const later
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

// will change to const later
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

// will change to const later
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

// will change to const later
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

// will change to const later
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

// will change to const later
pub const LOOKUP_PATH_WITHOUT_END: [[Bitboard; 64]; 64] = {
    let mut lookup: [[Bitboard; 64]; 64] =  [[Bitboard(0); 64]; 64];

    let mut from_sq: usize = 0;
    while from_sq < 64 {

        let (x, y) = (from_sq % 8, from_sq / 8);

        // make macro to easy all loops
        macro_rules! arm_loop {
            ( $back_cond:expr, $rem_its:expr, $sub:expr, $step:expr ) => {
                let mut path: u64 = 1 << from_sq;
                let mut to_sq: usize = from_sq;
                let mut remaining_its: usize = $rem_its;
                while remaining_its > 0 {

                    if $sub {
                        to_sq -= $step
                    } else {
                        to_sq += $step;
                    }
                    
                    lookup[from_sq][to_sq] = Bitboard(path);
                    
                    path |= 1 << to_sq;
                    remaining_its -= 1;
                }
            };
        }

        // r arm
        arm_loop!( x>0 , 7-x, false, 1 );

        // rd arm
        arm_loop!( x>0&&y<7 , min(7-x, y) , true , 7 );

        // d arm
        arm_loop!( y<7 , y , true, 8 );

        // ld
        arm_loop!( x<7&&y<7 , min(x, y) , true , 9);

        // l arm
        arm_loop!( x<7 , x , true , 1 );

        // lu arm
        arm_loop!( x<7&&y>0 , min(x, 7-y) , false , 7);

        // u arm
        arm_loop!( y>0 , 7-y , false , 8);

        // ru arm
        arm_loop!( x>0&&y>0 , min(7-x, 7-y) , false , 9);

        from_sq += 1;
    }

    lookup
};

// // the x_ray mask for each pair of (king square, slider square, slider type)
// const X_RAY_MASK: [[[Bitboard; 64]; 64]; 1+1+1] = {
//     todo!();
// };