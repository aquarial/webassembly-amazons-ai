// @ts-check
/* jshint -W069, esversion:6 */

import * as wasm from "amazons-ai-webassembly";
import { GameBoard, Player, GameState, Pos } from "./game.js";
import { DrawState } from "./drawstate.js";

/** @type {((dt: number, totaltime: number) => void)[]} */
let animations = [];


{
  /** @type {HTMLCanvasElement} */
  let canvas = (document.getElementById("thecanvas"));
  let c2d = canvas.getContext("2d");

  let gameboard = new GameBoard(8, 8);
  gameboard.addPlayer(new Player(new Pos(3, 3), "red"));
  gameboard.addPlayer(new Player(new Pos(3, 6), "red"));
  gameboard.addPlayer(new Player(new Pos(6, 3), "blue"));
  gameboard.addPlayer(new Player(new Pos(6, 6), "blue"));


  let gamestate = new GameState();

  let drawstate = new DrawState();

  /** @type {HTMLButtonElement} */
  let undo = (document.getElementById("undo"));
  undo.onmousedown = function (event) {
    gamestate.undoMove(gameboard);
    drawstate.piece = null;
    drawstate.move = null;
  }
  /** @type {HTMLButtonElement} */
  let makeai = (document.getElementById("makeai"));
  makeai.onmousedown = function (event) {
    let board = wasm.RequestedBoard.new();
    board.set_height(gameboard.height);
    board.set_width(gameboard.width);
    for (let y = 1; y <= gameboard.height; y++) {
      for (let x = 1; x <= gameboard.width; x++) {
        let at = gameboard.atYX(y, x);
        if (at instanceof Player) {
          board.add_player(y, x)
        } else if (at != undefined) {
          board.add_block(y, x)
        }
      }
    }
    board.board_greet();
    board.free();
    // wasm.greet();
    // gamestate.addMove(drawstate.piece, drawstate.move, tpos)
    // gameboard.makePlayerMove(drawstate.piece, drawstate.move, tpos)
  }


  canvas.onmouseleave = function (event) {
    drawstate.piece = null
    drawstate.move = null
    drawstate.mouse_pos.y = -1;
    drawstate.mouse_pos.x = -1;
  }

  // will be used for animations
  canvas.onmousemove = (function (event) {
    let tilesize = canvas.width / gameboard.width;

    let tx = Math.floor(event.offsetX / tilesize) + 1;
    let ty = Math.floor(event.offsetY / tilesize) + 1;
    drawstate.mouse_pos.x = tx;
    drawstate.mouse_pos.y = ty;
  })

  // click handler
  canvas.onmousedown = function (event) {
    let tilesize = canvas.width / gameboard.width;

    let tx = Math.floor(event.offsetX / tilesize) + 1;
    let ty = Math.floor(event.offsetY / tilesize) + 1;
    let tpos = new Pos(ty, tx);

    let at = gameboard.atPos(tpos);
    if (at instanceof Player) {
      if (at === drawstate.piece) {
        if (drawstate.move != null) {
          // placing a stone on the location of the moving piece
          gamestate.addMove(drawstate.piece, drawstate.move, tpos)
          gameboard.makePlayerMove(drawstate.piece, drawstate.move, tpos)
          drawstate.piece = null
          drawstate.move = null;
        } else {
          // re-click to deselect
          drawstate.piece = null
        }
      } else {
        drawstate.piece = null;
        if (at.team === gamestate.next_to_go) {
          drawstate.piece = at;
        }
        drawstate.move = null;
      }
    } else if (at != null) {
      drawstate.piece = null;
      drawstate.move = null;
    }

    if (at == undefined) {
      if (drawstate.piece == null) { // select piece
        // make pieces flash
      } else if (drawstate.move == null) { // move pieces
        if (gameboard.openLineTo(drawstate.piece, tpos)) {
          drawstate.move = tpos;
        } else {
          drawstate.piece = null;
        }
      } else { // place stone
        gameboard.blocked.set(drawstate.piece.pos.str(), undefined);
        if (gameboard.openLineTo(drawstate.move, tpos)) {
          gameboard.blocked.set(drawstate.piece.pos.str(), drawstate.piece);
          gamestate.addMove(drawstate.piece, drawstate.move, tpos)
          gameboard.makePlayerMove(drawstate.piece, drawstate.move, tpos)
          drawstate.piece = null;
          drawstate.move = null;
        } else {
          gameboard.blocked.set(drawstate.piece.pos.str(), drawstate.piece);
          drawstate.piece = null;
          drawstate.move = null;
        }
      }
    }


  }
  animations.push((dt, totaltime) => {
    drawstate.drawTiles(c2d, gameboard, canvas.width / gameboard.width)
  })
}


{
  let marker = 0;
  animations.push((dt, totaltime) => {
    if (totaltime > marker) {
      //console.log(marker, "seconds");
      marker += 5;
    }
  });
}



let previous = performance.now();
let totaltime = 0;

function animLoop() {
  let next = performance.now();
  let deltaTime = (next - previous) / 1000.0;
  totaltime += deltaTime;
  previous = next;

  animations.forEach(it => {
    it(deltaTime, totaltime);
  });

  window.requestAnimationFrame(animLoop);
}
animLoop();
