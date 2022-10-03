//
// const X_SHAPE: usize = 19;
// const Y_SHAPE: usize = 19;
// const N_SQUARES: usize = X_SHAPE * Y_SHAPE;
// static EXP_COEFF: f32 = 0.25;
//
// type BoardArray<T> = [[T; Y_SHAPE]; X_SHAPE];
//
// #[derive(default)]
// struct Node<'a> {
//     pub total_values: BoardArray<f32>,
//     pub n_visits: BoardArray<usize>,
//     pub priors: BoardArray<f32>,  // search probs for mcts
//     pub children: BoardArray<Option<&'a mut Node<'a>>>,  //allow for rollouts
//     pub parent: Option<&'a mut Node<'a>>  //allow for backprop
// }
//
// impl Node {
//     fn get_best_action(&self) -> (usize, usize) {
//         let mut argmax: (usize, usize) = (0, 0);
//         let mut max: f32 = f32::NEG_INFINITY;
//
//         for i in 0..X_SHAPE {
//             for j in 0..Y_SHAPE {
//
//                 let sum: usize = self.n_visits.iter().map(|a| a.iter().sum()).sum();
//                 q_val = self.total_values[i][j] / self.n_visits[i][j];
//                 q_val += EXP_COEFF * self.priors[i][j] * sum.sqrt() / (1 + self.n_visits[i][j]);
//
//                 if val > max {
//                     max = val;
//                     argmax = (i, j);
//                 }
//             }
//         }
//
//         return (i, j)
//     }
//
//     fn do_rollout(&mut self) -> () {
//
//     }
//
//     fn find_priors(&mut self) {
//         assert!(false);
//
//         for i in 0..X_SHAPE {
//             for j in 0..Y_SHAPE {
//                 self.priors[i][j] = 1.0 / N_SQUARES
//             }
//         }
//     }
//
//     fn make_child(&mut self, position: (usize, usize)) -> () {
//
//         let mut new_node: Node = Node::default();
//         new_node.find_priors();
//         self.children[position.0][position.1] = Some(&mut new_node);
//
//         new_node.do_rollout();
//
//
//         assert!(false);
//         ()
//     }
//
//     fn mcts_step(&mut self) -> () {
//         let (i, j): (usize, usize) = self.best_action();
//
//         match self.children[i][j] {
//             None => self.make_child((i, j)),
//             Some(child) => child.mcts_step()
//         }
//     }
// }