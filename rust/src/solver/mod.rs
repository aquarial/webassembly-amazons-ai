pub mod algo;
pub mod board;
pub mod compact_board;

use board::*;
use compact_board::*;

enum HistoryMove {
  NewGame(Board),
  Move(Move),
}

/// Data structures for amazon simulation,
/// history-tracking, and AI.
pub struct Amazons {
  current: Board,
  history: Vec<HistoryMove>,
  cache: DistState,
}

impl Amazons {
  pub fn new() -> Amazons {
    let b = Board::new();
    Amazons {
      current: b.clone(),
      history: vec![HistoryMove::NewGame(b)],
      cache: DistState::new(),
    }
  }

  /// Revert the last move.
  pub fn undo_move(&mut self) {
    match self.history.pop() {
      Some(HistoryMove::NewGame(b)) => {
        self.current = b;
      }
      Some(HistoryMove::Move(m)) => {
        self.current.un_apply_move(m);
      }
      None => {}
    }
  }

  /// Reference to current board
  pub fn current(&self) -> &CompactBoard {
    &self.current()
  }

  /// Try to record a player's move
  ///
  /// Return Err(msg) explaining the error if the move is invalid.
  pub fn player_move(&mut self, mv: Move) {
    self.current.apply_move(mv);
    self.history.push(HistoryMove::Move(mv));
    self.history.truncate(100);
  }

  /// Compute and make a move for an AI team.
  ///
  /// Return false if the AI gives up.
  pub fn ai_move(&mut self, team: Team) -> Option<CompactMove> {
    // TODO Multi-threading based on # of caches
    let cache = &mut self.cache;
    return match algo::min_max(cache, &self.current, team, 3) {
      (Some(m_move), _) => {
        self.current.apply_move(&m_move);
        self.history.push(HistoryMove::Move(m_move.clone()));
        self.history.truncate(100);
        Some(m_move)
      }
      (None, _) => None,
    };
  }
}

// #[cfg(test)]
// mod tests {
//   use crate::solver::*;

//   #[test]
//   fn ai_move() {
//     let b = CompactBoard::new(6, vec![
//       Player{pos:Pos{row:1,col:1},team:Team::Red},
//       Player{pos:Pos{row:2,col:2},team:Team::Blue},
//     ]);
//     let mut a = Amazons::from_board(b);
//     println!("{:?}", a.ai_move(Team::Red));
//     assert_eq!(2 + 2, 4);
//   }
// }
