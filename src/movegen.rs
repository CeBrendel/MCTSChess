use crate::chess::Bitboard;
use crate::compile_time_constants::*;
use crate::pext_consts::*;


pub struct MoveGenInfo <
    const CURRENT_PLAYER_IS_WHITE: bool,
    const HAS_EN_PASSANT: bool,
    const HAS_LONG_WHITE_RIGHTS: bool,
    const HAS_SHORT_WHITE_RIGHTS: bool,
    const HAS_LONG_BLACK_RIGHTS: bool,
    const HAS_SHORT_BLACK_RIGHTS: bool
> {
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
        bitloop!(self.own_pawns(), sq => {

        });

        // bishop / x-queen moves
        // or queens and bishops together? Or keep info on which piece is moving?

        // rook / plus-queen moves
        // or queens and rooks together? Or keep info on which piece is moving?

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

    pub fn get_checkmask(&self) -> Bitboard {
        // path from enemy to king (excluding king, including enemy)

        // get king sq
        let king_sq: usize = self.own_king().tzcnt();

        // empty check mask
        let mut check_mask: Bitboard = Bitboard(0);
        check_mask.visualize();


        // check by pawns
        check_mask |= Self::ENEMY_PAWNS_KING_ATTACKS[king_sq] & self.enemy_pawns();

        println!("After pawns:");
        check_mask.visualize();


        // checks by knights
        check_mask |= KNIGHT_MASK[king_sq] & self.enemy_knights();
        println!("After knights:");
        check_mask.visualize();


        // check bishops and x-queen
        {
            // relevant squares / occupancy
            let occupancy: usize = self.occupied.pext(LOOKUP_X_NO_FRAME_NO_CENTER[king_sq]);
            let possible_attackers: Bitboard = LOOKUP_X_ATTACK_PEXT[king_sq][occupancy];

            // check if there are enemy "X" sliders
            let attackers: Bitboard = possible_attackers & (self.enemy_bishops() | self.enemy_queens());

            // add path of attackers to checkmask (excluding king sq, including enemy sq)
            bitloop!{attackers, sq => {
                check_mask |= LOOKUP_PATH_WITHOUT_END[sq][king_sq];
            }}

            println!("After bihops&xqueens:");
            check_mask.visualize();
        }

        // check rooks and plus-queen
        {
            // relevant squares / occupancy
            let occupancy: usize = self.occupied.pext(LOOKUP_PLUS_NO_FRAME_NO_CENTER[king_sq]);
            let possible_attackers: Bitboard = LOOKUP_PLUS_ATTACK_PEXT[king_sq][occupancy];

            // check if there are enemy "+"" sliders
            let attackers: Bitboard = possible_attackers & (self.enemy_rooks() | self.enemy_queens());

            // add path of attackers to checkmask (excluding king sq, including enemy sq)
            bitloop!{attackers, sq => {
                check_mask |= LOOKUP_PATH_WITHOUT_END[sq][king_sq];
            }}

            println!("After rooks&+queens:");
            check_mask.visualize();
        }

        // n_checks??
        if !check_mask.has_bits() {
            check_mask = Bitboard(0xFFFFFFFFFFFFFFFF);
        }

        return check_mask;
    }


    fn get_pinmask(&self) -> Bitboard {
        // path from enemy to king (excluding king, including enemy) but all 1s if no check is present
        // ATTENTION: Sq behind king for rooky stuff
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
    const fn enemy_bishops(&self) -> Bitboard {
        if CURRENT_PLAYER_IS_WHITE {self.b_bishops} else {self.w_bishops}
    }

    #[inline(always)]
    const fn enemy_rooks(&self) -> Bitboard {
        if CURRENT_PLAYER_IS_WHITE {self.b_rooks} else {self.w_rooks}
    }

    #[inline(always)]
    const fn enemy_queens(&self) -> Bitboard {
        if CURRENT_PLAYER_IS_WHITE {self.b_queens} else {self.w_queens}
    }

    #[inline(always)]
    const fn own_king(&self) -> Bitboard {
        if CURRENT_PLAYER_IS_WHITE {self.w_kings} else {self.b_kings}
    }

    const ENEMY_PAWNS_KING_ATTACKS: &[Bitboard; 64] = &if CURRENT_PLAYER_IS_WHITE {BLACK_PAWN_KING_ATTACKS} else {WHITE_PAWN_KING_ATTACKS};
}