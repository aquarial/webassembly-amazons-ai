pub mod board;

use board::*;
use smallvec::SmallVec;
use std::collections::VecDeque;


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

  /// Revert the last two moves.
  pub fn undo_move(&mut self) {
    match self.history.pop() {
      Some(HistoryMove::NewGame(b)) => {
        self.current = b;
      },
      Some(HistoryMove::Move(m)) => {
        self.current.un_apply_move(&m);
      },
      None => {
      },
    }
  }

  /// All the pieces owned by a team.
  pub fn team_pieces<'s>(&'s self, team: Team) -> impl Iterator<Item=Pos> + 's {
    self.current.players()
      .filter(move |p| p.team == team)
      .map(|p| p.pos)
  }

  /// Try to record a player's move
  ///
  /// Return Err(msg) explaining the error if the move is invalid.
  pub fn player_move(&mut self, team: Team, pos: Pos, mv: Pos, shot: Pos) -> Result<(), String> {
    let board = self.current.clone();

    for &coord in &[pos, mv, shot] {
      if coord.row >= board.board_size || coord.col >= board.board_size {
        return Err(format!("Coord {:?} is outside board_size ({}, {})", coord,
                           board.board_size, board.board_size));
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
    return match max_move(&self.current, team, 3, &mut self.cache) {
      (Some(m_move), _) => {
        self.current.apply_move(&m_move);
        self.history.push(HistoryMove::Move(m_move.clone()));
        self.history.truncate(100);
        Some(m_move)
      }
      (None, _) => {
        None
      }
    }
  }
}

fn max_move(board: &Board, team: Team, depth: i32, cache: &mut DistState) -> (Option<CompactMove>, i64) {
  let mut local_board = board.clone();

  if depth <= 1 {
    let best = board.successors(team)
      .map(|mv| {
        local_board.apply_move(&mv);
        let eval = evaluate_by_queen_bfs_distance(&local_board, team.other(), cache);
        local_board.un_apply_move(&mv);
        (eval, mv)
      })
      .min_by_key(|it| it.0);
    if let Some((score, mv)) = best {
      return (Some(mv), score);
    } else {
      return (None, i64::min_value() + 1);
    }
  }

  let mut best: Option<CompactMove> = None;
  let mut score: i64 = i64::min_value() + 1;

  let top_boards = top_n(board.successors(team).map(|m| {
    local_board.apply_move(&m);
    let eval = evaluate_by_queen_bfs_distance(&local_board, team, cache);
    local_board.un_apply_move(&m);
    return (eval, m);
  }));

  for (_, b) in top_boards {
    local_board.apply_move(&b);
    let (_, resp_score) = max_move(&local_board, team.other(), depth-1, cache);
    local_board.un_apply_move(&b);

    if score < -resp_score {
      score = -resp_score;
      best = Some(b);
    }
  }

  match best {
    None => max_move(board, team, 1, cache),
    _ => (best, score)

  }
}

fn top_n<A>(iter: impl Iterator<Item = (i64, A)>) -> SmallVec<[(i64, A); 15]> {
  let mut vec = SmallVec::<[(i64, A); 15]>::new();

  iter.for_each(|new| {
    match vec.binary_search_by_key(& -new.0, |a| -a.0) {
      Ok(i) => vec.insert(i, new),
      Err(i) => vec.insert(i, new),
    }
    vec.truncate(14)
  });

  return vec;
}
