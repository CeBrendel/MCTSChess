
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Player {
    None, Black, White
}

impl Player {
    pub fn swap(self) -> Player {
        match self {
            Player::None => Player::None,
            Player::Black => Player::White,
            Player::White => Player::Black
        }
    }
}

pub trait TwoPlayerGame {
    fn current_player(&self) -> Player;
    fn eval(&self) -> (f32, bool);
    fn get_legal_actions(&self) -> Vec<usize>;
    fn step(&mut self, action: usize) -> ();
    fn undo(&mut self) -> ();
}

pub fn negamax<T: TwoPlayerGame>(game: &mut T, depth: usize) -> (f32, Option<usize>) {

    // eval game state and check if done
    let eval: (f32, bool) = game.eval();
    let (value, done) = eval;

    // if depth is 0 or game is done, return current value and no action
    return if depth == 0 || done {
        let coeff: f32 = match game.current_player() {
            Player::Black => -1f32,
            Player::White => 1f32,
            _ => panic!("")
        };
        (coeff * value, None)
    } else {
        // else loop through all possible next game states and keep track of best one
        let mut best_val: f32 = f32::MIN;
        let mut best_action: Option<usize> = None;

        // get legal moves and iterate over them
        for action in game.get_legal_actions() {

            // make move
            game.step(action);

            // recursive call
            let new_val = f32::max(
                best_val, - negamax(game, depth - 1).0
            );

            // if value of call is better than current value, remember it and the action
            if new_val > best_val {
                best_val = new_val;
                best_action = Some(action);
            }

            // undo move
            game.undo()
        }

        (best_val, best_action)
    }
}


pub fn negamax_alpha_beta<T: TwoPlayerGame>(
    game: &mut T, depth: usize, mut alpha: f32, beta: f32
) -> (f32, Option<usize>) {

    // eval game state and check if done
    let eval: (f32, bool) = game.eval();
    let (value, done) = eval;

    // if depth is 0 or game is done, return current value and no action
    return if depth == 0 || done {
        let coeff: f32 = match game.current_player() {
            Player::Black => -1f32,
            Player::White => 1f32,
            _ => panic!("")
        };
        (coeff * value, None)
    } else {
        // else loop through all possible next game states and keep track of best one
        let mut best_val: f32 = f32::MIN;
        let mut best_action: Option<usize> = None;

        // get legal moves and iterate over them
        for action in game.get_legal_actions() {

            // make move
            game.step(action);

            // recursive call
            let (recursive_value, _) = negamax_alpha_beta(
                game, depth - 1, -beta, -alpha
            );
            let new_val = f32::max(best_val, - recursive_value);

            // update alpha
            alpha = f32::max(alpha, new_val);

            // if value of call is better than current value, remember it and the action
            if new_val > best_val {
                best_val = new_val;
                best_action = Some(action);
            }

            // undo move
            game.undo();

            // check for cutoff
            if alpha >= beta {
                break;
            }
        }

        (best_val, best_action)
    }
}

