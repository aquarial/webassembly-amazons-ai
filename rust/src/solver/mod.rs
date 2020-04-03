pub mod board;

use board::*;
use smallvec::SmallVec;


/// Data structures for amazon simulation,
/// history-tracking, and AI.
pub struct Amazons {
  board_size: i8,
  boards: Vec<Board>,
  cache: DistState,
}


impl Amazons {
  pub fn from_board(board: Board) -> Amazons {
    Amazons {
      board_size: board.board_size,
      cache: DistState::with_board_size(board.board_size),
      boards: vec![board],
    }
  }
  pub fn new_5x5() -> Amazons {
    let board_size = 5 + 2;

    let mut players = Vec::new();
    players.push(Player{ team:Team::Red, pos:Pos {row:  2, col:  2} });
    players.push(Player{ team:Team::Red, pos:Pos {row:  2, col:  4} });
    players.push(Player{ team:Team::Blue, pos:Pos {row:  4, col:  2} });
    players.push(Player{ team:Team::Blue, pos:Pos {row:  4, col:  4} });

    Amazons {
      board_size: board_size,
      boards: vec![Board::new(board_size, players)],
      cache: DistState::with_board_size(board_size),
    }
  }

  pub fn new_8x8() -> Amazons {
    let board_size = 8 + 2;

    let mut players = Vec::new();
    players.push(Player{ team:Team::Red, pos:Pos {row:  3, col:  3} });
    players.push(Player{ team:Team::Red, pos:Pos {row:  3, col:  6} });
    players.push(Player{ team:Team::Blue, pos:Pos {row:  6, col:  3} });
    players.push(Player{ team:Team::Blue, pos:Pos {row:  6, col:  6} });

    Amazons {
      board_size: board_size,
      boards: vec![Board::new(board_size, players)],
      cache: DistState::with_board_size(board_size),
    }
  }

  /// Revert the last two moves.
  pub fn undo_2_move(&mut self) {
    if self.boards.len() >= 3 {
      self.boards.pop();
      self.boards.pop();
    }
  }

  /// All the pieces owned by a team.
  pub fn team_pieces<'s>(&'s self, team: Team) -> impl Iterator<Item=Pos> + 's {
    self.boards[self.boards.len() - 1].players()
      .filter(move |p| p.team == team)
      .map(|p| p.pos)
  }

  /// Try to record a player's move
  ///
  /// Return Err(msg) explaining the error if the move is invalid.
  pub fn player_move(&mut self, team: Team, pos: Pos, mv: Pos, shot: Pos) -> Result<(), String> {
    let board = self.boards[self.boards.len() - 1].clone();

    for &coord in &[pos, mv, shot] {
      if coord.row >= self.board_size || coord.col >= self.board_size {
        return Err(format!("Coord {:?} is outside board_size ({}, {})", coord,
                           self.board_size, self.board_size));
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
        let mut b = board.clone();
        b.apply_move(&CompactMove {
          player_ix: pi,
          old_pos: pos,
          new_pos: mv,
          new_shot: shot,
        });
        self.boards.push(b);
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
    let c0 = &mut self.cache;
    let mut board = self.boards[self.boards.len() - 1].clone();
    return match max_move(&board, team, EvalStrategy::QueenDistance, 3, c0) {
      (Some(b), _) => {
        board.apply_move(&b);
        self.boards.push(board);
        Some(b)
      }
      (None, _) => {
        None
      }
    }
  }

  /// Look back in history for a board state.
  ///
  /// If the index is too far back in time, this
  /// returns the first board in history.
  pub fn nth_last_board(&self, i: usize) -> Board {
    if self.boards.len() > i {
      return self.boards[self.boards.len() - 1 - i].clone();
    } else {
      return self.boards[0].clone();
    }
  }

  /// The most recent board state.
  pub fn curr_board(&self) -> &Board {
    return &self.boards[self.boards.len() - 1];
  }
}

fn max_move(board: &Board, team: Team, strategy: EvalStrategy, depth: i32, cache: &mut DistState) -> (Option<CompactMove>, i64) {
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
    let (_, resp_score) = max_move(&local_board, team.other(), strategy, depth-1, cache);
    local_board.un_apply_move(&b);

    if score < -resp_score {
      score = -resp_score;
      best = Some(b);
    }
  }

  match best {
    None => max_move(board, team, strategy, 1, cache),
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
