mod utils;
mod solver;

use solver::*;
use solver::board::*;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
  // fn alert(s: &str);
}

#[wasm_bindgen]
pub struct RequestedBoard {
  pub size:   f64,
  blocks:  Vec<(f64, f64)>,
  red_team: Vec<(f64, f64)>,
  blue_team: Vec<(f64, f64)>,
}

#[wasm_bindgen]
impl RequestedBoard {
  pub fn add_red_team(&mut self, y:f64, x:f64) {self.red_team.push((y,x));}
  pub fn add_blue_team(&mut self, y:f64, x:f64) {self.blue_team.push((y,x));}
  pub fn add_block(&mut self,  y:f64, x:f64) {self.blocks.push((y,x));}

  pub fn new() -> RequestedBoard {
    RequestedBoard {
      size: 0.0,
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

    for &(y,x) in &self.red_team {
      if !is_int_in_range(y, (1.0, self.size)) || !is_int_in_range(x, (1.0, self.size)) {
        return false;
      }
    }

    for &(y,x) in &self.blue_team {
      if !is_int_in_range(y, (1.0, self.size)) || !is_int_in_range(x, (1.0, self.size)) {
        return false;
      }
    }

    for &(y,x) in &self.blocks {
      if !is_int_in_range(y, (1.0, self.size)) || !is_int_in_range(x, (1.0, self.size)) {
        return false;
      }
    }

    return true;
  }
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
  let mut board = Board::new(rb.size as i8, players);
  for &(y,x) in &rb.blocks {
    board.wall_set(Pos {row:y as i8,col:x as i8}, true);
  }

  let mut amazon = Amazons::from_board(board);
  if amazon.ai_move(Team::Red) {
    let m = amazon.compute_last_move();
    return ReturnedMove {
      piece_y: m.player.pos.row as f64,
      piece_x: m.player.pos.col as f64,
      move_y: m.new_pos.row as f64,
      move_x: m.new_pos.col as f64,
      stone_y: m.new_shot.row as f64,
      stone_x: m.new_shot.col as f64,
    }
  }

  return ReturnedMove {
    piece_y: 0.0, piece_x: 0.0,
    move_y: 0.0, move_x: 0.0,
    stone_y: 0.0, stone_x: 0.0,
  };
}

#[wasm_bindgen]
pub struct ReturnedMove {
  pub piece_y: f64,
  pub piece_x: f64,
  pub move_y: f64,
  pub move_x: f64,
  pub stone_y: f64,
  pub stone_x: f64,
}

fn is_int_in_range(val: f64, range:(f64, f64)) -> bool {
  if !(range.0 <= val && val <= range.1) {
    return false;
  }
  if val.floor() != val {
    return false;
  }
  return true;
}
