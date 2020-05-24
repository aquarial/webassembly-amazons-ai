/// Red or Blue.
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

/// 3 positions that represent how the piece moves
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Move {
  pub old_pos: Pos,
  pub new_pos: Pos,
  pub new_shot: Pos,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BoardSlot {
  Empty,
  Wall,
  Piece(Team),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
  pub board: Vec<Vec<BoardSlot>>,
}

impl Board {
  pub fn new() -> Self {
    let board_size = 8 + 2; // 8x8 with walls
    let mut tokens = vec![vec![BoardSlot::Empty; board_size]; board_size];

    for r in 0..board_size {
      for c in 0..board_size {
        if r == 0 || c == 0 || r == board_size - 1 || c == board_size - 1 {
          tokens[r][c] = BoardSlot::Wall;
        }
      }
    }

    tokens[3][3] = BoardSlot::Piece(Team::Red);
    tokens[3][6] = BoardSlot::Piece(Team::Red);
    tokens[6][3] = BoardSlot::Piece(Team::Blue);
    tokens[6][6] = BoardSlot::Piece(Team::Blue);

    return Board { board: tokens };
  }

  pub fn pprint(&self) -> String {
    let mut s = String::new();
    for row in &self.board {
      for p in row {
        s.push(match p {
          BoardSlot::Empty => '.',
          BoardSlot::Wall => '#',
          BoardSlot::Piece(Team::Red) => 'R',
          BoardSlot::Piece(Team::Blue) => 'B',
        });
      }
      s.push('\n');
    }
    return s;
  }

  pub fn size(&self) -> usize {
    self.board.len().max(self.board.iter().map(|r| r.len()).max().unwrap())
  }

  pub fn open_line_along(&mut self, start: Pos, end: Pos) -> bool {
    if !start.in_a_line_with(end) { return false; };
    if start == end { return false; }
    return start
      .along_line(end).into_iter()
      .all(|p| *self.at(p) == BoardSlot::Empty)
  }

  pub fn at(&mut self, pos: Pos) -> &mut BoardSlot {
    &mut self.board[pos.row as usize][pos.col as usize]
  }

  pub fn apply_move(&mut self, mv: Move) {
    let t = self.at(mv.old_pos).clone();
    *self.at(mv.old_pos) = BoardSlot::Empty;
    *self.at(mv.new_pos) = t;
    *self.at(mv.new_shot) = BoardSlot::Wall;
  }

  pub fn un_apply_move(&mut self, mv: Move) {
    let t = self.at(mv.new_pos).clone();
    *self.at(mv.new_pos) = BoardSlot::Empty;
    *self.at(mv.old_pos) = t;
    *self.at(mv.new_shot) = BoardSlot::Empty;
  }
}
