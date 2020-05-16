mod solver;
mod utils;

use solver::board::*;
use solver::*;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
  // fn alert(s: &str);

  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[wasm_bindgen]
pub struct State {
  gamestate: Amazons,
  drawstate: DrawableBoard,
  turn: Team,
}

#[wasm_bindgen]
impl State {
  pub fn new() -> State {
    let board_size = 8 + 2;
    let mut players = Vec::new();
    players.push(Player{ team:Team::Red, pos:Pos {row:  2, col:  2} });
    players.push(Player{ team:Team::Red, pos:Pos {row:  2, col:  4} });
    players.push(Player{ team:Team::Blue, pos:Pos {row:  4, col:  2} });
    players.push(Player{ team:Team::Blue, pos:Pos {row:  4, col:  4} });
    let amazons = Amazons::from_board(Board::new(board_size, players));

    let drawboard = DrawableBoard::new(amazons.current());

    State {
      turn: Team::Red,
      gamestate: amazons,
      drawstate: drawboard,
    }
  }

  pub fn mouse_move(&self, row: f64, col: f64) {
    if !is_int_in_range(row, (1.0, self.size() as f64))
      || !is_int_in_range(col, (1.0, self.size() as f64)) {
      log(&format!("State.mouse_move({}, {}) out of (0, {}) range!",
        row, col, self.size()));
    }
    self.drawstate.mouse_move(row as usize, col as usize);
  }

  pub fn turn(&self) -> Team {
    self.turn
  }

  pub fn size(&self) -> i8 {
    self.gamestate.current().board_size - 2
  }

  pub fn token(&self, row: f64, col: f64) -> DrawableToken {
    if !is_int_in_range(row, (1.0, self.size() as f64))
      || !is_int_in_range(col, (1.0, self.size() as f64)) {
      log(&format!("State.token({}, {}) out of [1, {}) range!",
        row, col, self.size()));
    }
    self.drawstate.board[row as usize][col as usize]
  }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawableToken {
  pub wall: bool,
  pub piece: Team,
  pub hover: bool,
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DrawableBoard {
  board: Vec<Vec<DrawableToken>>,
  mouse: Pos,
}

impl DrawableBoard {
  pub fn new(from: &Board) -> DrawableBoard {
    let dt = DrawableToken {
      wall: false,
      piece: Team::Red,
      hover: false,
    };
    let mut tokens = vec![vec![dt; from.board_size as usize]; from.board_size as usize];

    for r in 0..from.board_size {
      for c in 0..from.board_size {
        if from.wall_at(Pos { row: r, col: c }) {
          tokens[r as usize][c as usize].wall = true;
        }
      }
    }
    for p in from.players() {
      tokens[p.pos.row as usize][p.pos.col as usize].piece = p.team;
    }
    DrawableBoard {
      board: tokens,
      mouse: Pos { row: 0, col: 0 },
    }
  }

  pub fn mouse_move(&self, row: usize, col: usize) {
    if row == self.mouse.row as usize && col == self.mouse.col as usize {
      return;
    }
  }
}

#[wasm_bindgen]
pub struct RequestedBoard {
  pub size: f64,
  blocks: Vec<(f64, f64)>,
  red_team: Vec<(f64, f64)>,
  blue_team: Vec<(f64, f64)>,
}

#[wasm_bindgen]
impl RequestedBoard {
  pub fn add_red_team(&mut self, y: f64, x: f64) {
    self.red_team.push((y, x));
  }
  pub fn add_blue_team(&mut self, y: f64, x: f64) {
    self.blue_team.push((y, x));
  }
  pub fn add_block(&mut self, y: f64, x: f64) {
    self.blocks.push((y, x));
  }

  pub fn new(size: f64) -> RequestedBoard {
    RequestedBoard {
      size: size,
      red_team: Vec::new(),
      blue_team: Vec::new(),
      blocks: Vec::new(),
    }
  }

  pub fn is_valid(&self) -> bool {
    // [1, width]  [1, height]
    if !is_int_in_range(self.size, (1.0, 10.0)) {
      return false;
    }

    for &(y, x) in &self.red_team {
      if !is_int_in_range(y, (1.0, self.size)) || !is_int_in_range(x, (1.0, self.size)) {
        return false;
      }
    }

    for &(y, x) in &self.blue_team {
      if !is_int_in_range(y, (1.0, self.size)) || !is_int_in_range(x, (1.0, self.size)) {
        return false;
      }
    }

    for &(y, x) in &self.blocks {
      if !is_int_in_range(y, (1.0, self.size)) || !is_int_in_range(x, (1.0, self.size)) {
        return false;
      }
    }

    return true;
  }
}

fn is_int_in_range(val: f64, range: (f64, f64)) -> bool {
  if !(range.0 <= val && val <= range.1) {
    return false;
  }
  if val.floor() != val {
    return false;
  }
  return true;
}

#[wasm_bindgen]
pub struct ReturnedMove {
  pub piece_y: i8,
  pub piece_x: i8,
  pub move_y: i8,
  pub move_x: i8,
  pub stone_y: i8,
  pub stone_x: i8,
}

#[wasm_bindgen]
pub fn compute_ai_move(rb: &RequestedBoard) -> ReturnedMove {
  let mut players = Vec::new();
  for &(y,x) in &rb.blue_team {
    players.push(Player{ team:Team::Blue, pos:Pos {row:  y as i8, col:  x as i8} });
  }
  for &(y,x) in &rb.red_team {
    players.push(Player{ team:Team::Red, pos:Pos {row:  y as i8, col:  x as i8} });
  }
  let mut board = Board::new(rb.size as i8 + 2, players);
  for &(y,x) in &rb.blocks {
    board.wall_set(Pos {row:y as i8,col:x as i8}, true);
  }

  let mut amazon = Amazons::from_board(board);

  if let Some(cm) = amazon.ai_move(Team::Red) {
    // log(&amazon.nth_last_board(1).pprint());
    // log(&amazon.nth_last_board(0).pprint());
    return ReturnedMove {
      piece_y: cm.old_pos.row,
      piece_x: cm.old_pos.col,
      move_y: cm.new_pos.row,
      move_x: cm.new_pos.col,
      stone_y: cm.new_shot.row,
      stone_x: cm.new_shot.col,
    };
  } else {
    return ReturnedMove {
      piece_y: 0, piece_x: 0,
      move_y: 0, move_x: 0,
      stone_y: 0, stone_x: 0,
    };
  }
}
