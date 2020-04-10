
use crate::board::*;
use smallvec::SmallVec;
use std::collections::VecDeque;


pub fn smart_min_max(cache: &mut DistState, board: &Board, team: Team) -> (Option<CompactMove>, i64) {
  let mut local_board = board.clone();

  let mut moves = Vec::<(CompactMove, i64)>::new();

  for m in board.successors(team) {
    local_board.apply_move(&m);
    let eval = -evaluate_by_queen_bfs_distance(&local_board, team.other(), cache);
    local_board.un_apply_move(&m);

    let mut other = (-1, eval);
    for (ix, (cm, cv)) in moves.iter().enumerate() {
      if m.old_pos == cm.old_pos && m.new_pos == cm.new_pos {
        other = (ix as i64, *cv);
        break;
      }
    }
    if other.0 == -1 {
      moves.push((m, eval));
      moves.truncate(4);
    } else if eval > other.1 {
      moves[other.0 as usize] = (m, eval);
    }
  }

  let mut best: Option<CompactMove> = None;
  let mut score: i64 = i64::min_value() + 1;

  for (b, _) in moves {
    local_board.apply_move(&b);
    let (_, resp_score) = min_max(cache, &local_board, team.other(), 2);
    local_board.un_apply_move(&b);

    if score < -resp_score {
      score = -resp_score;
      best = Some(b);
    }
  }

  match best {
    None => min_max(cache, board, team, 1),
    _ => (best, score)
  }
}



pub fn min_max(cache: &mut DistState, board: &Board, team: Team, depth: i32) -> (Option<CompactMove>, i64) {
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
    let (_, resp_score) = min_max(cache, &local_board, team.other(), depth-1);
    local_board.un_apply_move(&b);

    if score < -resp_score {
      score = -resp_score;
      best = Some(b);
    }
  }

  match best {
    None => min_max(cache, board, team, 1),
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
