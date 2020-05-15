pub mod algo;
pub mod board;

use board::*;

enum HistoryMove {
  NewGame(Board),
  Move(CompactMove),
}

/// Data structures for amazon simulation,
/// history-tracking, and AI.
pub struct Amazons {
  current: Board,
  history: Vec<HistoryMove>,
  cache: DistState,
}

impl Amazons {
  pub fn from_board(board: Board) -> Amazons {
    Amazons {
      current: board.clone(),
      history: vec![HistoryMove::NewGame(board)],
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
        self.current.un_apply_move(&m);
      }
      None => {}
    }
  }

  /// Reference to current board
  pub fn current(&self) -> &Board {
    &self.current()
  }

  /// All the pieces owned by a team.
  pub fn team_pieces(&self, team: Team) -> Vec<Pos> {
    self
      .current
      .players()
      .filter(move |p| p.team == team)
      .map(|p| p.pos)
      .collect()
  }

  /// Try to record a player's move
  ///
  /// Return Err(msg) explaining the error if the move is invalid.
  pub fn player_move(&mut self, team: Team, pos: Pos, mv: Pos, shot: Pos) -> Result<(), String> {
    let board = self.current.clone();

    for &coord in &[pos, mv, shot] {
      if coord.row >= board.board_size || coord.col >= board.board_size {
        return Err(format!(
          "Coord {:?} is outside board_size ({}, {})",
          coord, board.board_size, board.board_size
        ));
      }
    }
    if pos == mv || mv == shot || !pos.in_a_line_with(mv) {
      return Err(format!("Moves not in a line!"));
    }
    if !mv.in_a_line_with(shot) {
      return Err(format!("Shoot is not in a line!"));
    }
    if let Some(er) = pos.along_line(mv).iter().find(|&&p| board.wall_at(p)) {
      return Err(format!("Can't move through piece at {:?}", er));
    }
    if let Some(er) = mv.along_line(shot).iter().filter(|&&p| p != pos).find(|&&p| board.wall_at(p)) {
      return Err(format!("Can't place token through piece at {:?}", er));
    }
    if let Some((pi, p)) = board.players().enumerate().find(|(_,play)| play.pos == pos) {
      if p.team == team {
        let new_move = CompactMove {
          player_ix: pi,
          old_pos: pos,
          new_pos: mv,
          new_shot: shot,
        };
        self.current.apply_move(&new_move);
        self.history.push(HistoryMove::Move(new_move));
        self.history.truncate(100);
        return Ok(());
      }
    }
    return Err(format!("You don't have a piece at the position"));
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

#[cfg(test)]
mod tests {
  use crate::solver::*;

  #[test]
  fn ai_move() {
    let b = Board::new(6, vec![
      Player{pos:Pos{row:1,col:1},team:Team::Red},
      Player{pos:Pos{row:2,col:2},team:Team::Blue},
    ]);
    let mut a = Amazons::from_board(b);
    println!("{:?}", a.ai_move(Team::Red));
    assert_eq!(2 + 2, 4);
  }
}
