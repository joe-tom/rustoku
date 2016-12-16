use std::thread;
use std::cmp;

mod board;


fn main () {

}

fn minimax (board: board::Board,mut alpha: i32,mut beta: i32, depth: u32, you: bool) {
  let val = board.evaluate();
  if val == INF || val == N_INF || depth == 0 {
    return val;
  }

  if you {
    let mut v = N_INF;
    let moves = board.get_moves(you);

    for mov in &moves {
      board.make_move(mov);
      v = cmp::max(v, minimax(board, alpha, beta, depth - 1, !you));
      board.undo_move(mov);
      alpha = cmp::max(alpha, v);
      if beta <= alpha {
        break;
      }
    }

    return v;
  } else {
    let mut v = INF;
    let moves = board.get_moves(you);

    for mov in &moves {
      board.make_move(mov);
      v = cmp::min(v, minimax(board, alpha, beta, depth - 1, !you));
      board.undo_move(mov);
      beta = cmp::min(beta, v);
      if beta <= alpha {
        break;
      }
    }

    return v;
  }
}
