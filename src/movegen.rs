use crate::chess::Bitboard;
use crate::compile_time_constants::*;


struct MoveGenInfo <
    const CURRENT_PLAYER_IS_WHITE: bool,
    const HAS_EN_PASSANT: bool,
    const HAS_LONG_WHITE_RIGHTS: bool,
    const HAS_SHORT_WHITE_RIGHTS: bool,
    const HAS_LONG_BLACK_RIGHTS: bool,
    const HAS_SHORT_BLACK_RIGHTS: bool
> {
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
    occupied: Bitboard
}

impl <
    const CURRENT_PLAYER_IS_WHITE: bool,
    const HAS_EN_PASSANT: bool,
    const HAS_LONG_WHITE_RIGHTS: bool,
    const HAS_SHORT_WHITE_RIGHTS: bool,
    const HAS_LONG_BLACK_RIGHTS: bool,
    const HAS_SHORT_BLACK_RIGHTS: bool
> MoveGenInfo <
    CURRENT_PLAYER_IS_WHITE,
    HAS_EN_PASSANT,
    HAS_LONG_WHITE_RIGHTS,
    HAS_SHORT_WHITE_RIGHTS,
    HAS_LONG_BLACK_RIGHTS,
    HAS_SHORT_BLACK_RIGHTS  
> {
    pub fn get_legal_moves(&self) -> Vec<usize> {

        let check_mask: Bitboard = self.get_checkmask();

        // knight moves
        bitloop!(self.own_knights(), sq => {

            // pinmask for not leaving king in check
            let x_pin_mask: Bitboard = Bitboard(0x0);
            let plus_pin_mask: Bitboard = Bitboard(0x0);

            // knight pattern & enemy or empty & pinmask & checkmask
            let legal_moves: Bitboard = KNIGHT_MASK[sq] & self.enemy_or_empty() & (x_pin_mask | plus_pin_mask).neg() & check_mask;

        });

        // pawn moves (en passant)

        // bishop / x-queen moves

        // rook / plus-queen moves

        // king moves (castling)



        // match self.get_n_checks() {
        //     2 => {// kingmoves only

        //     },  
        //     1 => {// blocking & taking & moving the king
        //         let _checkmask: Bitboard = self.get_checkmask();
        //         let _pinmask: Bitboard = self.get_pinmask();
        //     }
        //     0 => {// everything that doesn't leave the king in check
        //         let _checkmask: Bitboard = Bitboard(0xFFFFFFFFFFFFFFFF);
        //         let _pinmask: Bitboard = self.get_pinmask();
        //     },
        //     _ => {}
        // }

        todo!();
    }

    #[inline(always)]
    const fn get_n_checks(&self) -> usize {
        todo!();
    }

    fn get_checkmask(&self) -> Bitboard {
        // path from enemy to king (excluding king, including enemy)
        compiletime_toggleable_assert!(self.get_n_checks() == 1);

        // get king sq
        let king_sq: usize = self.own_king().tzcnt();

        // empty check mask
        let mut check_mask: Bitboard = Bitboard(0);

        // check by pawns
        check_mask = check_mask | Self::ENEMY_PAWNS_KING_ATTACKS[king_sq];

        // checks by knights
        check_mask = check_mask | (KNIGHT_MASK[king_sq] & self.enemy_knights());

        // check bishops and x-queen

        // check rooks and plus-queen

        // n_checks
        if !check_mask.has_bits() {
            check_mask = Bitboard(0xFFFFFFFFFFFFFFFF);
        }
        todo!();
    }


    fn get_pinmask(&self) -> Bitboard {
        // path from enemy to king (excluding king, including enemy) but all Ones if no check is present
        // split into HV and D12?
        todo!();
    }
    
    
    #[inline(always)]
    const fn enemy_or_empty(&self) -> Bitboard {
        if CURRENT_PLAYER_IS_WHITE {self.white_mask.neg()} else {self.black_mask.neg()}
    }
    
    #[inline(always)]
    const fn own_pawns(&self) -> Bitboard {
        if CURRENT_PLAYER_IS_WHITE {self.w_pawns} else {self.b_pawns}
    }

    #[inline(always)]
    const fn enemy_pawns(&self) -> Bitboard {
        if CURRENT_PLAYER_IS_WHITE {self.b_pawns} else {self.w_pawns}
    }

    #[inline(always)]
    const fn own_knights(&self) -> Bitboard {
        if CURRENT_PLAYER_IS_WHITE {self.w_knights} else {self.b_knights}
    }

    #[inline(always)]
    const fn enemy_knights(&self) -> Bitboard {
        if CURRENT_PLAYER_IS_WHITE {self.b_knights} else {self.w_knights}
    }

    #[inline(always)]
    const fn own_king(&self) -> Bitboard {
        if CURRENT_PLAYER_IS_WHITE {self.w_kings} else {self.b_kings}
    }

    const ENEMY_PAWNS_KING_ATTACKS: &[Bitboard; 64] = &if CURRENT_PLAYER_IS_WHITE {BLACK_PAWN_KING_ATTACKS} else {WHITE_PAWN_KING_ATTACKS};
}