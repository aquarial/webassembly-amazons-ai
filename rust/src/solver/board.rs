use bv::BitVec;
use std::collections::VecDeque;
use wasm_bindgen::prelude::*;

/// Red or Blue.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Team {
  Red,
  Blue,
}
impl Team {
  pub fn teams() -> Vec<Team> {
    return vec![Team::Red, Team::Blue];
  }

  pub fn other(&self) -> Team {
    match self {
      Team::Red => Team::Blue,
      Team::Blue => Team::Red,
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pos {
  pub row: i8,
  pub col: i8,
}
impl Pos {
  /// Whether this position is axis-aligned
  /// with the other.
  pub fn in_a_line_with(&self, other: Pos) -> bool {
    let dr = self.row - other.row;
    let dc = self.col - other.col;

    dr == 0 || dc == 0 || dr.abs() == dc.abs()
  }

  /// A path from `self` to `other`, which may
  /// walk diagonally.
  pub fn along_line(&self, other: Pos) -> Vec<Pos> {
    let mut v = Vec::new();
    let mut walk = self.clone();
    while walk != other {
      if walk.col < other.col {
        walk.col += 1;
      }
      if walk.col > other.col {
        walk.col -= 1;
      }
      if walk.row < other.row {
        walk.row += 1;
      }
      if walk.row > other.row {
        walk.row -= 1;
      }
      v.push(walk);
    }
    return v;
  }

  /// Coordinate flatten.
  pub fn to_linear(&self, num_cols: i8) -> usize {
    self.row as usize * num_cols as usize + self.col as usize
  }

  /// Calculate `self + (dir × dist)`
  pub fn with_offset(&self, dir: (i8, i8), dist: i8) -> Pos {
    Pos {
      row: self.row + dist * dir.0,
      col: self.col + dist * dir.1,
    }
  }
}

/// Team and location.
///
/// NOTE: a player with Pos={0, 0} is considered invalid
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Player {
  pub team: Team,
  pub pos: Pos,
}

/// Player and what they do.
#[derive(Clone, Debug)]
pub struct CompactMove {
  pub player_ix: usize,
  pub old_pos: Pos,
  pub new_pos: Pos,
  pub new_shot: Pos,
}

#[derive(Clone, Debug)]
pub struct DistState {
  left: Vec<u8>,
  right: Vec<u8>,
  next: VecDeque<(Pos, u8)>,
}
impl DistState {
  pub fn new() -> DistState {
    DistState {
      left: Vec::new(),
      right: Vec::new(),
      next: VecDeque::new(),
    }
  }
}

const MAX_NUM_PLAYERS: usize = 4;

/// Game state at an instant.
#[derive(Clone, Debug)]
pub struct Board {
  walls: BitVec,
  pub board_size: i8,
  players_array: [Player; MAX_NUM_PLAYERS],
}

impl Board {
  pub fn new(board_size: i8, players: Vec<Player>) -> Board {
    let mut b = BitVec::new_fill(false, (board_size * board_size) as u64);

    for r in 0..board_size {
      for c in 0..board_size {
        if r == 0 || c == 0 || r == board_size - 1 || c == board_size - 1 {
          b.set((r * board_size + c) as u64, true);
        }
      }
    }
    for p in &players {
      b.set(p.pos.to_linear(board_size) as u64, true);
    }

    assert!(players.len() <= MAX_NUM_PLAYERS);
    assert!(players.len() >= 1);
    let mut pa = [Player { pos: Pos {row:0, col:0}, team: Team::Red }; MAX_NUM_PLAYERS];
    for (pi, p) in players.into_iter().enumerate() {
      assert!(p.pos != Pos { row: 0, col: 0 });
      pa[pi] = p;
    }

    return Board {
      walls: b,
      board_size: board_size,
      players_array: pa,
    };
  }

  pub fn wall_set(&mut self, p: Pos, val: bool) {
    self.walls.set(p.to_linear(self.board_size) as u64, val);
  }
  pub fn wall_at(&self, p: Pos) -> bool {
    self.walls.get((p.to_linear(self.board_size)) as u64)
  }

  pub fn pprint(&self) -> String {
    let mut s = String::new();
    for r in 0..self.board_size {
      for c in 0..self.board_size {
        let pos = Pos { row: r, col: c };
        match self.players().find(|p| p.pos == pos) {
          Some(p) => {
            if !self.wall_at(pos) {
              if p.team == Team::Blue {
                s.push('b');
              } else {
                s.push('w');
              }
            } else {
              if p.team == Team::Blue {
                s.push('B');
              } else {
                s.push('W');
              }
            }
          }
          None => {
            if self.wall_at(pos) {
              s.push('#')
            } else {
              s.push('.');
            }
          }
        }
      }
      s.push('\n');
    }
    return s;
  }

  pub fn players(&self) -> impl Iterator<Item = &Player> {
    self
      .players_array
      .iter()
      .filter(|p| p.pos != Pos { row: 0, col: 0 })
  }

  pub fn apply_move(&mut self, mv: &CompactMove) {
    // NOTE: this only works since valid players are before the
    // invalid players in the array, thus the indexes match
    self.wall_set(mv.old_pos, false);
    self.wall_set(mv.new_pos, true);
    self.wall_set(mv.new_shot, true);
    self.players_array[mv.player_ix].pos = mv.new_pos;
  }

  pub fn un_apply_move(&mut self, mv: &CompactMove) {
    // NOTE: this only works since valid players are before the
    // invalid players in the array, thus the indexes match
    self.wall_set(mv.new_pos, false);
    self.wall_set(mv.new_shot, false);
    self.wall_set(mv.old_pos, true);
    self.players_array[mv.player_ix].pos = mv.old_pos;
  }

  pub fn successors<'a>(&'a self, team: Team) -> impl Iterator<Item = CompactMove> + 'a {
    self
      .players()
      .enumerate()
      .filter(move |(_, player)| player.team == team)
      .flat_map(move |(pi, player): (usize, &'a Player)| {
        queen_range(self, player.pos, player.pos).flat_map(move |pos: Pos| {
          queen_range(self, pos, player.pos).map(move |shot: Pos| {
            CompactMove {
              player_ix: pi,
              old_pos: player.pos,
              new_pos: pos,
              new_shot: shot,
            }
          })
        })
      })
  }
}

pub fn evaluate_by_queen_bfs_distance(board: &Board, team: Team, dist_state: &mut DistState) -> i64 {
  bfs(board, team, &mut dist_state.next, &mut dist_state.left);
  bfs(board, team.other(), &mut dist_state.next, &mut dist_state.right);

  let mut score = 0;
  let mut is_end = true;
  for (&a, &b) in dist_state.left.iter().zip(dist_state.right.iter()) {
    if a < b {
      score = score + 1;
    }
    if a > b {
      score = score - 1;
    }
    if a != u8::max_value() && b != u8::max_value() {
      is_end = false;
    }
  }
  if is_end {
    if score >= 0 {
      return i64::max_value();
    }
    return i64::min_value() + 1;
  }
  return score;
}

fn bfs(board: &Board, team: Team, next: &mut VecDeque<(Pos, u8)>, distances: &mut Vec<u8>) {
  for i in 0..distances.len() {
    distances[i] = u8::max_value();
  }
  while distances.len() < (board.board_size * board.board_size) as usize {
    distances.push(u8::max_value());
  }
  next.clear();
  board
    .players()
    .filter(|p| p.team == team)
    .map(|p| (p.pos, 0))
    .for_each(|it| next.push_back(it));

  while let Some((pos, depth)) = next.pop_front() {
    for neigh in queen_range(board, pos, pos) {
      let place = &mut distances[neigh.to_linear(board.board_size)];
      if depth + 1 < *place {
        *place = depth + 1;
        next.push_back((neigh, depth + 1));
      }
    }
  }
}

const QUEEN_DIRS: [(i8,i8); 8] = [(-1,-1),(-1,0),(-1,1),
                                  ( 0,-1)       ,( 0,1),
                                  ( 1,-1),( 1,0),( 1,1)];


fn queen_range<'a>(board: &'a Board, from: Pos, blank: Pos) -> impl Iterator<Item = Pos> + 'a {
  QUEEN_DIRS.iter().flat_map(move |dir| {
    (1..)
      .map(move |dist| from.with_offset(*dir, dist))
      .take_while(move |place| !board.wall_at(*place) || *place == blank)
  })
}
