

#![allow(dead_code)]
#![allow(unused_imports)]

#![feature(const_eval_limit)]
#![const_eval_limit = "0"]

extern crate lazy_static;
extern crate rand;
use min_max::TwoPlayerGame;

mod connect4;
mod checkers;
mod tree;
mod min_max;
mod chess;
mod movegen;
mod compile_time_constants;
mod pext_consts;

fn main() {

    use crate::chess::*;
    use crate::compile_time_constants::*;
    use crate::pext_consts::*;

    // let sq: usize = 27;
    // let pattern = LOOKUP_X_NO_FRAME_NO_CENTER[sq];
    
    // let occupancy = Bitboard(0x200 | 0x6280 | 0x2000000000000);
    // let pext = occupancy.unsafe_pext(pattern);
    // let unfolded = unfold_occupancy_x(sq, pext);
    // let attacks = attacked_by_x(sq, pext);

    // occupancy.visualize();
    // println!("Sq: ({}, {}), pext: {}", sq%8, sq/8, pext);
    // unfolded.visualize();
    // attacks.visualize();
    
    // // // pattern.visualize();

    // assert!(occupancy & pattern == unfolded);

    // path from enemy to king (including king and square behind, excluding enemy)
    // let LOOKUP_KING_ENEMY_PATH: [[Bitboard; 64]; 64] = {
    //     let mut lookup: [[Bitboard; 64]; 64] = [[Bitboard(0); 64]; 64];

    //     let mut king_sq: usize = 0;
    //     while king_sq < 64 {

    //         let (x, y) = (king_sq % 8, king_sq / 8);

    //         // make macro to easy all loops
    //         macro_rules! arm_loop {
    //             ( $back_cond:expr, $rem_its:expr, $sub:expr, $step:expr ) => {
    //                 let mut path: u64 = 1 << king_sq;
    //                 if $back_cond {path |= 1 << (if !$sub {king_sq - $step} else {king_sq + $step})}
    //                 let mut remaining_its: usize = $rem_its;
    //                 let mut enemy_sq: usize = king_sq;
    //                 while remaining_its > 0 {

    //                     if $sub {
    //                         enemy_sq -= $step
    //                     } else {
    //                         enemy_sq += $step;
    //                     }
                        
    //                     lookup[king_sq][enemy_sq] = Bitboard(path);
                        
    //                     path |= 1 << enemy_sq;
    //                     remaining_its -= 1;
    //                 }
    //             };
    //         }

    //         // r arm
    //         arm_loop!( x>0 , 7-x, false, 1 );

    //         // rd arm
    //         arm_loop!( x>0&&y<7 , usize::min(7-x, y) , true , 7 );

    //         // d arm
    //         arm_loop!( y<7 , y , true, 8 );

    //         // ld
    //         arm_loop!( x<7&&y<7 , usize::min(x, y) , true , 9);

    //         // l arm
    //         arm_loop!( x<7 , x , true , 1 );

    //         // lu arm
    //         arm_loop!( x<7&&y>0 , usize::min(x, 7-y) , false , 7);

    //         // u arm
    //         arm_loop!( y>0 , 7-y , false , 8);

    //         // ru arm
    //         arm_loop!( x>0&&y>0 , usize::min(7-x, 7-y) , false , 9);

    //         king_sq += 1;
    //     }

    //     lookup
    // };


    // let king_sq: usize = 28;
    // for sq in 0..64 {
    //     println!("KingSq: {}, EnemySq: {}", king_sq, sq);
    //     LOOKUP_KING_ENEMY_PATH[king_sq][sq].visualize();
    // }

}
