
// use crate::chess::Bitboard;
// use crate::compile_time_constants::*;

// const fn get_proper_frame(sq: usize) -> Bitboard {
//     let (x, y) = (sq % 8, sq / 8);
//     return match (x, y) {
//         (x, y) if 0<x && x<7 && 0<y && y<7 => Bitboard(0xFF818181818181FF),
//         (0, y) if 0<y && y<7 => Bitboard(0xFF808080808080FF),
//         (7, y) if 0<y && y<7 => Bitboard(0xFF010101010101FF),
//         (x, 0) if 0<x && x<7 => Bitboard(0xFF81818181818181),
//         (x, 7) if 0<x && x<7 => Bitboard(0x81818181818181FF),
//         (0, 0) => Bitboard(0xFF80808080808080),
//         (0, 7) => Bitboard(0x80808080808080FF),
//         (7, 0) => Bitboard(0xFF01010101010101),
//         (7, 7) => Bitboard(0x01010101010101FF),
//         (_, _) => panic!("")
//     }
// }

// pub const LOOKUP_PLUS_NO_FRAME_NO_CENTER: [Bitboard; 64] = {
//     let mut lookup: [Bitboard; 64] = [Bitboard(0); 64];

//     let mut sq = 0;
//     while sq < 64 {
//         let frame: Bitboard = get_proper_frame(sq);
//         lookup[sq] = PLUS_MASK[sq].and(frame.neg()).and(Bitboard(!(1 << sq)));
//         sq += 1;
//     }

//     lookup
// };

// pub const LOOKUP_X_NO_FRAME_NO_CENTER: [Bitboard; 64] = {
//     let mut lookup: [Bitboard; 64] = [Bitboard(0); 64];

//     let mut sq = 0;
//     while sq < 64 {
//         let frame: Bitboard = get_proper_frame(sq);
//         lookup[sq] = X_MASK[sq].and(frame.neg()).and(Bitboard(!(1 << sq)));
//         sq += 1;
//     }

//     lookup
// };

// const fn inverse_pext(pext: usize, mut mask: Bitboard) -> Bitboard {
//     let mut unfolded: u64 = 0;

//     let mut cnt: usize = 0;
//     bitloop!(
//         mask,
//         sq => {
//             if pext & (1 << cnt) != 0 {
//                 unfolded |= 1u64 << sq;
//             }
//             cnt += 1;
//         }
//     );

//     return Bitboard(unfolded);
// }

// const fn unfold_occupancy_plus(sq: usize, occupancy: usize) -> Bitboard {
//     let relevant_squares: Bitboard = LOOKUP_PLUS_NO_FRAME_NO_CENTER[sq];
//     return inverse_pext(occupancy, relevant_squares);
// }

// const fn unfold_occupancy_x(sq: usize, occupancy: usize) -> Bitboard {
//     let relevant_squares: Bitboard = LOOKUP_X_NO_FRAME_NO_CENTER[sq];
//     return inverse_pext(occupancy, relevant_squares);
// }

// const fn attacked_by_plus(sq: usize, occupancy: usize) -> Bitboard {

//     // unfold occupancy (inverse is PEXT)
//     let occupancy: Bitboard = unfold_occupancy_plus(sq, occupancy);

//     // empty board to register attacked squares
//     let mut attacked_sqs: u64 = 0;

//     // get coordinates of sq
//     let (x, y) = (sq % 8, sq / 8);

//     // right arm
//     let mut remaining_its: usize = 7 - x; // how many iterations the arm has
//     let mut walking_sq: usize = sq;
//     while remaining_its > 0 {
//         walking_sq += 1;
//         attacked_sqs |= 1 << walking_sq; // set bit
//         remaining_its -= 1;
//         if occupancy.has_entry_at(walking_sq) {break}; // if sq is occupied, break
//     }

//     // left arm
//     let mut remaining_its: usize = x;
//     let mut walking_sq: usize = sq;
//     while remaining_its > 0 {
//         walking_sq -= 1;
//         attacked_sqs |= 1 << walking_sq;
//         remaining_its -= 1;
//         if occupancy.has_entry_at(walking_sq) {break};
//     }

//     // lower arm
//     let mut remaining_its: usize = y;
//     let mut walking_sq: usize = sq;
//     while remaining_its > 0 {
//         walking_sq -= 8;
//         attacked_sqs |= 1 << walking_sq;
//         remaining_its -= 1;
//         if occupancy.has_entry_at(walking_sq) {break};
//     }

//     // upper arm
//     let mut remaining_its: usize = 7 - y;
//     let mut walking_sq: usize = sq;
//     while remaining_its > 0 {
//         walking_sq += 8;
//         attacked_sqs |= 1 << walking_sq;
//         remaining_its -= 1;
//         if occupancy.has_entry_at(walking_sq) {break};
//     }

//     return Bitboard(attacked_sqs);
// }

// const fn min(a: usize, b: usize) -> usize {
//     // I can't belive I have to reimplement this. The original version isn't const
//     return if a > b {b} else {a};
// }

// const fn attacked_by_x(sq: usize, occupancy: usize) -> Bitboard {

//     // unfold occupancy (inverse is PEXT)
//     let occupancy: Bitboard = unfold_occupancy_x(sq, occupancy);

//     // empty board to register attacked squares
//     let mut attacked_sqs: u64 = 0;

//     // get coordinates of sq
//     let (x, y) = (sq % 8, sq / 8);

//     // ru arm
//     let mut remaining_its: usize = min(7 - x, 7 - y); // how many iterations the arm has
//     let mut walking_sq: usize = sq;
//     while remaining_its > 0 {
//         walking_sq += 9;
//         attacked_sqs |= 1 << walking_sq; // set bit
//         remaining_its -= 1;
//         if occupancy.has_entry_at(walking_sq) {break}; // if sq is occupied, break
//     }

//     // rd arm
//     let mut remaining_its: usize = min(7 - x, y);
//     let mut walking_sq: usize = sq;
//     while remaining_its > 0 {
//         walking_sq -= 7;
//         attacked_sqs |= 1 << walking_sq;
//         remaining_its -= 1;
//         if occupancy.has_entry_at(walking_sq) {break};
//     }

//     // ld arm
//     let mut remaining_its: usize = min(x, y);
//     let mut walking_sq: usize = sq;
//     while remaining_its > 0 {
//         walking_sq -= 9;
//         attacked_sqs |= 1 << walking_sq;
//         remaining_its -= 1;
//         if occupancy.has_entry_at(walking_sq) {break};
//     }

//     // lu arm
//     let mut remaining_its: usize = min(x, 7 - y);
//     let mut walking_sq: usize = sq;
//     while remaining_its > 0 {
//         walking_sq += 7;
//         attacked_sqs |= 1 << walking_sq;
//         remaining_its -= 1;
//         if occupancy.has_entry_at(walking_sq) {break};
//     }

//     return Bitboard(attacked_sqs);
// }

// // 64 squares x (10-12) bits of possible occupancies
// // pub const LOOKUP_PLUS_ATTACK_PEXT: [[Bitboard; 4096]; 64] = {
// //     let mut lookup: [[Bitboard; 4096]; 64] = [[Bitboard(0); 4096]; 64];

// //     let mut sq: usize = 0;
// //     while sq < 64 {
        
// //         // look up relevant squares (anywhere in between 10-12 squares depending on rook position)
// //         let relevant_squares: Bitboard = LOOKUP_PLUS_NO_FRAME_NO_CENTER[sq];
// //         let num_bits: usize = relevant_squares.number_of_set_bits();

// //         // for n possibly occupied squares there are 2^n = (1 << n) many configurations, look through all of them
// //         let num_possible_occupancies: usize = 1 << num_bits;
// //         let mut occupancy: usize = 0;
// //         while occupancy < num_possible_occupancies {

// //             // calculate squares that are attacked by the rook (based on their occupancy) and write into lookup
// //             lookup[sq][occupancy] = attacked_by_plus(sq, occupancy);

// //             occupancy += 1;
// //         }

// //         sq += 1;
// //     }

// //     lookup
// // };

// // 64 squares x (5-9) bits of possible occupancies
// // pub const LOOKUP_X_ATTACK_PEXT: [[Bitboard; 512]; 64] = {
// //     let mut lookup: [[Bitboard; 512]; 64] = [[Bitboard(0); 512]; 64];

// //     let mut sq: usize = 0;
// //     while sq < 64 {
        
// //         // look up relevant squares (anywhere in between 5-9 squares depending on bishop position)
// //         let relevant_squares: Bitboard = LOOKUP_X_NO_FRAME_NO_CENTER[sq];
// //         let num_bits: usize = relevant_squares.number_of_set_bits();

// //         // for n possibly occupied squares there are 2^n = (1 << n) many configurations, look through all of them
// //         let num_possible_occupancies: usize = 1 << num_bits;
// //         let mut occupancy: usize = 0;
// //         while occupancy < num_possible_occupancies {

// //             // calculate squares that are attacked by the rook (based on their occupancy) and write into lookup
// //             lookup[sq][occupancy] = attacked_by_x(sq, occupancy);

// //             occupancy += 1;
// //         }

// //         sq += 1;
// //     }

// //     lookup
// // };
