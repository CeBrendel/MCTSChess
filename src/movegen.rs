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

pub enum MoveGen {
    V000000(MoveGenInfo<false, false, false, false, false, false>),
    V000001(MoveGenInfo<false, false, false, false, false, true>),
    V000010(MoveGenInfo<false, false, false, false, true, false>),
    V000011(MoveGenInfo<false, false, false, false, true, true>),
    V000100(MoveGenInfo<false, false, false, true, false, false>),
    V000101(MoveGenInfo<false, false, false, true, false, true>),
    V000110(MoveGenInfo<false, false, false, true, true, false>),
    V000111(MoveGenInfo<false, false, false, true, true, true>),
    V001000(MoveGenInfo<false, false, true, false, false, false>),
    V001001(MoveGenInfo<false, false, true, false, false, true>),
    V001010(MoveGenInfo<false, false, true, false, true, false>),
    V001011(MoveGenInfo<false, false, true, false, true, true>),
    V001100(MoveGenInfo<false, false, true, true, false, false>),
    V001101(MoveGenInfo<false, false, true, true, false, true>),
    V001110(MoveGenInfo<false, false, true, true, true, false>),
    V001111(MoveGenInfo<false, false, true, true, true, true>),
    V010000(MoveGenInfo<false, true, false, false, false, false>),
    V010001(MoveGenInfo<false, true, false, false, false, true>),
    V010010(MoveGenInfo<false, true, false, false, true, false>),
    V010011(MoveGenInfo<false, true, false, false, true, true>),
    V010100(MoveGenInfo<false, true, false, true, false, false>),
    V010101(MoveGenInfo<false, true, false, true, false, true>),
    V010110(MoveGenInfo<false, true, false, true, true, false>),
    V010111(MoveGenInfo<false, true, false, true, true, true>),
    V011000(MoveGenInfo<false, true, true, false, false, false>),
    V011001(MoveGenInfo<false, true, true, false, false, true>),
    V011010(MoveGenInfo<false, true, true, false, true, false>),
    V011011(MoveGenInfo<false, true, true, false, true, true>),
    V011100(MoveGenInfo<false, true, true, true, false, false>),
    V011101(MoveGenInfo<false, true, true, true, false, true>),
    V011110(MoveGenInfo<false, true, true, true, true, false>),
    V011111(MoveGenInfo<false, true, true, true, true, true>),
    V100000(MoveGenInfo<true, false, false, false, false, false>),
    V100001(MoveGenInfo<true, false, false, false, false, true>),
    V100010(MoveGenInfo<true, false, false, false, true, false>),
    V100011(MoveGenInfo<true, false, false, false, true, true>),
    V100100(MoveGenInfo<true, false, false, true, false, false>),
    V100101(MoveGenInfo<true, false, false, true, false, true>),
    V100110(MoveGenInfo<true, false, false, true, true, false>),
    V100111(MoveGenInfo<true, false, false, true, true, true>),
    V101000(MoveGenInfo<true, false, true, false, false, false>),
    V101001(MoveGenInfo<true, false, true, false, false, true>),
    V101010(MoveGenInfo<true, false, true, false, true, false>),
    V101011(MoveGenInfo<true, false, true, false, true, true>),
    V101100(MoveGenInfo<true, false, true, true, false, false>),
    V101101(MoveGenInfo<true, false, true, true, false, true>),
    V101110(MoveGenInfo<true, false, true, true, true, false>),
    V101111(MoveGenInfo<true, false, true, true, true, true>),
    V110000(MoveGenInfo<true, true, false, false, false, false>),
    V110001(MoveGenInfo<true, true, false, false, false, true>),
    V110010(MoveGenInfo<true, true, false, false, true, false>),
    V110011(MoveGenInfo<true, true, false, false, true, true>),
    V110100(MoveGenInfo<true, true, false, true, false, false>),
    V110101(MoveGenInfo<true, true, false, true, false, true>),
    V110110(MoveGenInfo<true, true, false, true, true, false>),
    V110111(MoveGenInfo<true, true, false, true, true, true>),
    V111000(MoveGenInfo<true, true, true, false, false, false>),
    V111001(MoveGenInfo<true, true, true, false, false, true>),
    V111010(MoveGenInfo<true, true, true, false, true, false>),
    V111011(MoveGenInfo<true, true, true, false, true, true>),
    V111100(MoveGenInfo<true, true, true, true, false, false>),
    V111101(MoveGenInfo<true, true, true, true, false, true>),
    V111110(MoveGenInfo<true, true, true, true, true, false>),
    V111111(MoveGenInfo<true, true, true, true, true, true>)
}

impl MoveGen {
    pub fn get_checkmask(&self) -> Bitboard {
        match self {
            MoveGen::V000000(info) => info.get_checkmask(),
            MoveGen::V000001(info) => info.get_checkmask(),
            MoveGen::V000010(info) => info.get_checkmask(),
            MoveGen::V000011(info) => info.get_checkmask(),
            MoveGen::V000100(info) => info.get_checkmask(),
            MoveGen::V000101(info) => info.get_checkmask(),
            MoveGen::V000110(info) => info.get_checkmask(),
            MoveGen::V000111(info) => info.get_checkmask(),
            MoveGen::V001000(info) => info.get_checkmask(),
            MoveGen::V001001(info) => info.get_checkmask(),
            MoveGen::V001010(info) => info.get_checkmask(),
            MoveGen::V001011(info) => info.get_checkmask(),
            MoveGen::V001100(info) => info.get_checkmask(),
            MoveGen::V001101(info) => info.get_checkmask(),
            MoveGen::V001110(info) => info.get_checkmask(),
            MoveGen::V001111(info) => info.get_checkmask(),
            MoveGen::V010000(info) => info.get_checkmask(),
            MoveGen::V010001(info) => info.get_checkmask(),
            MoveGen::V010010(info) => info.get_checkmask(),
            MoveGen::V010011(info) => info.get_checkmask(),
            MoveGen::V010100(info) => info.get_checkmask(),
            MoveGen::V010101(info) => info.get_checkmask(),
            MoveGen::V010110(info) => info.get_checkmask(),
            MoveGen::V010111(info) => info.get_checkmask(),
            MoveGen::V011000(info) => info.get_checkmask(),
            MoveGen::V011001(info) => info.get_checkmask(),
            MoveGen::V011010(info) => info.get_checkmask(),
            MoveGen::V011011(info) => info.get_checkmask(),
            MoveGen::V011100(info) => info.get_checkmask(),
            MoveGen::V011101(info) => info.get_checkmask(),
            MoveGen::V011110(info) => info.get_checkmask(),
            MoveGen::V011111(info) => info.get_checkmask(),
            MoveGen::V100000(info) => info.get_checkmask(),
            MoveGen::V100001(info) => info.get_checkmask(),
            MoveGen::V100010(info) => info.get_checkmask(),
            MoveGen::V100011(info) => info.get_checkmask(),
            MoveGen::V100100(info) => info.get_checkmask(),
            MoveGen::V100101(info) => info.get_checkmask(),
            MoveGen::V100110(info) => info.get_checkmask(),
            MoveGen::V100111(info) => info.get_checkmask(),
            MoveGen::V101000(info) => info.get_checkmask(),
            MoveGen::V101001(info) => info.get_checkmask(),
            MoveGen::V101010(info) => info.get_checkmask(),
            MoveGen::V101011(info) => info.get_checkmask(),
            MoveGen::V101100(info) => info.get_checkmask(),
            MoveGen::V101101(info) => info.get_checkmask(),
            MoveGen::V101110(info) => info.get_checkmask(),
            MoveGen::V101111(info) => info.get_checkmask(),
            MoveGen::V110000(info) => info.get_checkmask(),
            MoveGen::V110001(info) => info.get_checkmask(),
            MoveGen::V110010(info) => info.get_checkmask(),
            MoveGen::V110011(info) => info.get_checkmask(),
            MoveGen::V110100(info) => info.get_checkmask(),
            MoveGen::V110101(info) => info.get_checkmask(),
            MoveGen::V110110(info) => info.get_checkmask(),
            MoveGen::V110111(info) => info.get_checkmask(),
            MoveGen::V111000(info) => info.get_checkmask(),
            MoveGen::V111001(info) => info.get_checkmask(),
            MoveGen::V111010(info) => info.get_checkmask(),
            MoveGen::V111011(info) => info.get_checkmask(),
            MoveGen::V111100(info) => info.get_checkmask(),
            MoveGen::V111101(info) => info.get_checkmask(),
            MoveGen::V111110(info) => info.get_checkmask(),
            MoveGen::V111111(info) => info.get_checkmask()
        }
    }

    // pub fn from_fen() -> Self {
    //     const CURRENT_PLAYER_IS_WHITE: bool = true;
    //     const HAS_EN_PASSANT: bool = true;
    //     const HAS_LONG_WHITE_RIGHTS: bool = true;
    //     const HAS_SHORT_WHITE_RIGHTS: bool = true;
    //     const HAS_LONG_BLACK_RIGHTS: bool = true;
    //     const HAS_SHORT_BLACK_RIGHTS: bool = true;

    //     match self {
    //         MoveGen::V000000(info),
    //         MoveGen::V000001(info),
    //         MoveGen::V000010(info),
    //         MoveGen::V000011(info),
    //         MoveGen::V000100(info),
    //         MoveGen::V000101(info),
    //         MoveGen::V000110(info),
    //         MoveGen::V000111(info),
    //         MoveGen::V001000(info),
    //         MoveGen::V001001(info),
    //         MoveGen::V001010(info),
    //         MoveGen::V001011(info),
    //         MoveGen::V001100(info),
    //         MoveGen::V001101(info),
    //         MoveGen::V001110(info),
    //         MoveGen::V001111(info),
    //         MoveGen::V010000(info),
    //         MoveGen::V010001(info),
    //         MoveGen::V010010(info),
    //         MoveGen::V010011(info),
    //         MoveGen::V010100(info),
    //         MoveGen::V010101(info),
    //         MoveGen::V010110(info),
    //         MoveGen::V010111(info),
    //         MoveGen::V011000(info),
    //         MoveGen::V011001(info),
    //         MoveGen::V011010(info),
    //         MoveGen::V011011(info),
    //         MoveGen::V011100(info),
    //         MoveGen::V011101(info),
    //         MoveGen::V011110(info),
    //         MoveGen::V011111(info),
    //         MoveGen::V100000(info),
    //         MoveGen::V100001(info),
    //         MoveGen::V100010(info),
    //         MoveGen::V100011(info),
    //         MoveGen::V100100(info),
    //         MoveGen::V100101(info),
    //         MoveGen::V100110(info),
    //         MoveGen::V100111(info),
    //         MoveGen::V101000(info),
    //         MoveGen::V101001(info),
    //         MoveGen::V101010(info),
    //         MoveGen::V101011(info),
    //         MoveGen::V101100(info),
    //         MoveGen::V101101(info),
    //         MoveGen::V101110(info),
    //         MoveGen::V101111(info),
    //         MoveGen::V110000(info),
    //         MoveGen::V110001(info),
    //         MoveGen::V110010(info),
    //         MoveGen::V110011(info),
    //         MoveGen::V110100(info),
    //         MoveGen::V110101(info),
    //         MoveGen::V110110(info),
    //         MoveGen::V110111(info),
    //         MoveGen::V111000(info),
    //         MoveGen::V111001(info),
    //         MoveGen::V111010(info),
    //         MoveGen::V111011(info),
    //         MoveGen::V111100(info),
    //         MoveGen::V111101(info),
    //         MoveGen::V111110(info),
    //         MoveGen::V111111(info)
    //     }
    // }
}