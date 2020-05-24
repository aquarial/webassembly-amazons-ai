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
  mouse: Pos,
  selected_piece: Option<Pos>,
  selected_move: Option<Pos>,
}

#[wasm_bindgen]
impl State {
  pub fn new() -> State {
    State {
      gamestate: Amazons::new(),
      mouse: Pos { row: 0, col: 0 },
      selected_piece: None,
      selected_move: None,
    }
  }

  fn clear_selected(&mut self) {
    self.selected_piece = None;
    self.selected_move = None;
  }

  pub fn turn(&self) -> DrawableTeam {
    self.gamestate.turn.into()
  }

  pub fn size(&self) -> usize {
    // remember: 8x8 board is internally a 10x10 with walls around it
    // so -2 to give the illusion it's a clear 8x8
    self.gamestate.current.size() - 2
  }

  pub fn new_game(&mut self) {
    self.gamestate.new_game();
    log(&self.gamestate.current.pprint());
  }

  pub fn ai_move(&mut self) {
    self.clear_selected();
    self.gamestate.ai_move();
  }

  pub fn undo(&mut self) {
    self.gamestate.undo_move();
    self.clear_selected();
  }

  pub fn mouse_leave(&mut self) {
    self.clear_selected();
    self.mouse = Pos { row: -1, col: -1 };
  }

  pub fn mouse_click(&mut self, row: f64, col: f64) {
    if !is_int_in_range(row, (1.0, self.size() as f64))
    || !is_int_in_range(col, (1.0, self.size() as f64)) {
    log(&format!("State.mouse_click({}, {}) out of [1, {}) range!",
      row, col, self.size()));
    }

    let clicked = Pos { row: row as i8, col: col as i8 };

    match (self.selected_piece, self.selected_move) {
      (None, _) => {},
      (Some(piece),  None) => {
        if self.gamestate.current.open_line_along(piece, clicked) {
          self.selected_move = Some(clicked);
          return;
        }
      },
      (Some(piece), Some(mv)) => {
        self.gamestate.current.swap_pos(piece, mv); // swap
        if self.gamestate.current.open_line_along(mv, clicked) {
          self.gamestate.current.swap_pos(piece, mv); // undo-swap
          self.clear_selected();
          self.gamestate.player_move(Move {
            old_pos: piece,
            new_pos: mv,
            new_shot: clicked,
          });
          return;
        }
        self.gamestate.current.swap_pos(piece, mv); // undo-swap
      }
    };

    self.clear_selected();
    if let BoardSlot::Piece(t) = self.gamestate.current.at(clicked) {
      if *t == self.gamestate.turn {
        self.selected_piece = Some(clicked);
      }
    }
  }

  pub fn mouse_move(&mut self, row: f64, col: f64) {
    if !is_int_in_range(row, (1.0, self.size() as f64))
      || !is_int_in_range(col, (1.0, self.size() as f64)) {
      log(&format!("State.mouse_move({}, {}) out of [1, {}) range!",
        row, col, self.size()));
    }
    self.mouse.row = row as i8;
    self.mouse.col = col as i8;
  }

  pub fn token(&mut self, row: f64, col: f64) -> DrawableToken {
    if !is_int_in_range(row, (1.0, self.size() as f64))
      || !is_int_in_range(col, (1.0, self.size() as f64)) {
      log(&format!("State.token({}, {}) out of [1, {}) range!",
        row, col, self.size()));
    }

    let location = Pos { row: row as i8, col: col as i8 };
    let mut dt = DrawableToken { wall: false, hover: false, piece: None};


    match (self.selected_piece, self.selected_move) {
      (None, _) => {
        match self.gamestate.current.at(location) {
          BoardSlot::Empty => {},
          BoardSlot::Wall => { dt.wall = true; },
          BoardSlot::Piece(t) => { dt.piece = Some(t.clone().into()); },
        };
      },
      (Some(piece),  None) => {

        match self.gamestate.current.at(location) {
          BoardSlot::Empty => {
            if location == self.mouse && self.gamestate.current.open_line_along(piece, location) {
              if let BoardSlot::Piece(t) = self.gamestate.current.at(piece) {
                dt.piece = Some(t.clone().into());
              }
              dt.hover = true;
            }
          },
          BoardSlot::Wall => { dt.wall = true; },
          BoardSlot::Piece(t) => {
            if piece == location {
              dt.hover = true;
            }
            dt.piece = Some(t.clone().into()); },
        };
      },
      (Some(piece), Some(mv)) => {

        self.gamestate.current.swap_pos(piece, mv);

        match self.gamestate.current.at(location) {
          BoardSlot::Empty => {
            if location == self.mouse && self.gamestate.current.open_line_along(mv, location) {
              dt.wall = true;
              dt.hover = true;
            } else if location == piece {
              //  after selecting a move, should the original piece still be highlighted?
              // dt.piece = Some(DrawableTeam::Red); // TODO FIXME AAAH
              // dt.hover = true;
            }
          },
          BoardSlot::Wall => { dt.wall = true; },
          BoardSlot::Piece(t) => {
            if location == mv {
              dt.hover = true;
            }
            dt.piece = Some(t.clone().into());
          }
        };

        self.gamestate.current.swap_pos(piece, mv);
      }
    };

    return dt;
  }
}


#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub enum DrawableTeam {
  Red, Blue,
}
impl From<Team> for DrawableTeam {
  fn from(t: Team) -> DrawableTeam {
    match t {
      Team::Red => DrawableTeam::Red,
      Team::Blue => DrawableTeam::Blue,
    }
  }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct DrawableToken {
  pub wall: bool,
  pub hover: bool,
  pub piece: Option<DrawableTeam>,
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


// #[wasm_bindgen]
// pub struct RequestedBoard {
//   pub size: f64,
//   blocks: Vec<(f64, f64)>,
//   red_team: Vec<(f64, f64)>,
//   blue_team: Vec<(f64, f64)>,
// }

// #[wasm_bindgen]
// impl RequestedBoard {
//   pub fn add_red_team(&mut self, y: f64, x: f64) {
//     self.red_team.push((y, x));
//   }
//   pub fn add_blue_team(&mut self, y: f64, x: f64) {
//     self.blue_team.push((y, x));
//   }
//   pub fn add_block(&mut self, y: f64, x: f64) {
//     self.blocks.push((y, x));
//   }

//   pub fn new(size: f64) -> RequestedBoard {
//     RequestedBoard {
//       size: size,
//       red_team: Vec::new(),
//       blue_team: Vec::new(),
//       blocks: Vec::new(),
//     }
//   }

//   pub fn is_valid(&self) -> bool {
//     // [1, width]  [1, height]
//     if !is_int_in_range(self.size, (1.0, 10.0)) {
//       return false;
//     }

//     for &(y, x) in &self.red_team {
//       if !is_int_in_range(y, (1.0, self.size)) || !is_int_in_range(x, (1.0, self.size)) {
//         return false;
//       }
//     }

//     for &(y, x) in &self.blue_team {
//       if !is_int_in_range(y, (1.0, self.size)) || !is_int_in_range(x, (1.0, self.size)) {
//         return false;
//       }
//     }

//     for &(y, x) in &self.blocks {
//       if !is_int_in_range(y, (1.0, self.size)) || !is_int_in_range(x, (1.0, self.size)) {
//         return false;
//       }
//     }

//     return true;
//   }
// }

// #[wasm_bindgen]
// pub fn compute_ai_move(rb: &RequestedBoard) -> ReturnedMove {
//   let mut players = Vec::new();
//   for &(y,x) in &rb.blue_team {
//     players.push(Player{ team:Team::Blue, pos:Pos {row:  y as i8, col:  x as i8} });
//   }
//   for &(y,x) in &rb.red_team {
//     players.push(Player{ team:Team::Red, pos:Pos {row:  y as i8, col:  x as i8} });
//   }
//   let mut board = CompactBoard::new(rb.size as i8 + 2, players);
//   for &(y,x) in &rb.blocks {
//     board.wall_set(Pos {row:y as i8,col:x as i8}, true);
//   }

//   let mut amazon = Amazons::from_board(board);

//   if let Some(cm) = amazon.ai_move(Team::Red) {
//     // log(&amazon.nth_last_board(1).pprint());
//     // log(&amazon.nth_last_board(0).pprint());
//     return ReturnedMove {
//       piece_y: cm.old_pos.row,
//       piece_x: cm.old_pos.col,
//       move_y: cm.new_pos.row,
//       move_x: cm.new_pos.col,
//       stone_y: cm.new_shot.row,
//       stone_x: cm.new_shot.col,
//     };
//   } else {
//     return ReturnedMove {
//       piece_y: 0, piece_x: 0,
//       move_y: 0, move_x: 0,
//       stone_y: 0, stone_x: 0,
//     };
//   }
// }
