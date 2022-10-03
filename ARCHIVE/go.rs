use std::sync::Arc;

const BOARD_SHAPE: (usize, usize) = (9, 9);
const X_SHAPE: usize = BOARD_SHAPE.0;
const Y_SHAPE: usize = BOARD_SHAPE.1;

type BoardArray<T: Default> = [[T; Y_SHAPE]; X_SHAPE];

fn default_array<T: Default>() -> BoardArray<T> {
    return BoardArray([[T::default(); Y_SHAPE]; X_SHAPE]);
}

type Position = (usize, usize);

#[derive(Copy, Clone)]
enum Player {
    Black, White, None
}

struct Chain {
    pub owned_by: Player,
    pub entries: BoardArray<bool>,
    pub liberties: BoardArray<bool>
}

impl Chain {
    fn union(mut self, other: Chain) -> Chain {

        assert!(self.player == other.player);

        // union liberties
        for x in 0..X_SHAPE {
            for y in 0..Y_SHAPE {
                // add liberty of other to self
                self.liberties[x][y] |= other.liberties[x][y];
            }
        }

        // union entries
        for x in 0..X_SHAPE {
            for y in 0..Y_SHAPE {

                // add entry of other to self
                self.entries[x][y] |= other.entries[x][y];

                // if (x, y) contains a stone, it is not a liberty
                if self.entries[x][y] {
                    self.liberties[x][y] = false;
                }
            }
        }

        // check liberties?

        return self;
    }

    fn union_at(mut chains: Vec<Chains>) -> Chain {

        // pop last chain to modify and return
        let mut return_chain: Chain = chains.pop()?;

        // union liberties
        for x in 0..X_SHAPE {
            for y in 0..Y_SHAPE {

                // check if we already have a liberty
                if return_chain.liberties[x][y] {
                    break;
                }

                // add liberties from other chains
                for chain in chains {
                    // if there is a liberty from the other chain, add and break
                    if chain.liberties[x][y] {
                        return_chain.liberties[x][y] = true;
                        break;
                    }
                }
            }
        }

        // union entries
        for x in 0..X_SHAPE {
            for y in 0..Y_SHAPE {

                // check if we already have a entry
                if return_chain.entries[x][y] {
                    break;
                }

                // add entries from other chains
                for chain in chains {
                    // if there is a entry from the other chain, add and break
                    if chain.entries[x][y] {
                        return_chain.entries[x][y] = true;
                        break;
                    }
                }

                // check if we now have a entry at (x, y) and if so remove liberty
                if return_chain.entries[x][y] {
                    return_chain.liberties[x][y] = false
                }

            }
        }

        return return_chain;
    }
}

// struct Game<'a> {
//     pub current_player: Player,
//     pub board: BoardArray<Player>,
//     pub chains: std::collections::HashMap<Chain, &mut Chain>,
//     pub zobrist_hash: usize,  // to detect ko
// }
//
// impl Game {
//
//     fn set_stone(&mut self, position: Position, player: Player) -> () {
//
//         // unpack future position
//         let (x, y) = position;
//
//         // get chains to union by filtering such that:
//         //   1. chain belongs to the player setting the stone
//         //   2. chain has new position as liberty
//         let union_chains: Vec<Chain> = self.chains
//             .iter()
//             .filter(|chain| chain.owned_by == player)
//             .filter(|chain| chain.liberties[x][y])
//             .map(|chain| **chain)
//             .collect();
//
//         // union found chains at new position
//         // let unioned_chain: Chain = Chain::union_at(position, union_chains);
//     }
//     // fn get_legal_moves(&self) -> BoardArray<bool> {
//     //
//     //     let mut legal_move_mask: BoardArray<bool> = default_array::<bool>();
//     //
//     //     let directions: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
//     //
//     //     for x in 0..X_SHAPE {
//     //         for y in 0..Y_SHAPE {
//     //
//     //             // check if position is empty
//     //             match self.board[x][y] {
//     //                 Black | White => break,
//     //                 None => {}
//     //             }
//     //
//     //             // check for suicide
//     //             let mut all_enemies: bool = true;
//     //             for (dx, dy) in directions {
//     //                 let occupancy: Player = self.board[x+dx][y+dy];
//     //                 if occupancy == Player::None || occupancy == self.current_player {
//     //                     all_enemies = False;
//     //                     break;
//     //                 }
//     //             }
//     //
//     //             // check for ko
//     //             assert!(false);
//     //         }
//     //     }
//     //
//     //     return legal_move_mask;
//     // }
// }