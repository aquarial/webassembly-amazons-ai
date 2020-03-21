// @ts-check
/* jshint -W069, esversion:6 */

import * as wasm from "amazons-ai-webassembly";
import { GameBoard, Player } from "./board.js";
import { DrawState } from "./drawstate.js";

/** @type {((dt: number, totaltime: number) => void)[]} */
let animations = [];


{
  /**
   * @param {CanvasRenderingContext2D} c2d
   * @param {GameBoard} gameboard
   */
  function drawTiles(c2d, gameboard) {
    let tilesize = canvas.width / gameboard.width;
    let checker_colors = ["brown", "gray"]

    for (let y = 1; y <= gameboard.height; y++) {
      for (let x = 1; x <= gameboard.width; x++) {
        c2d.fillStyle = checker_colors[(x + y) % 2]
        c2d.fillRect((x - 1) * tilesize, (y - 1) * tilesize, tilesize, tilesize)

        let at = gameboard.atPos(y, x)
        if (at instanceof Player) {
          c2d.fillStyle = at.team
          c2d.beginPath();
          c2d.ellipse((x - 1 + 0.5) * tilesize, (y - 1 + 0.5) * tilesize,
            tilesize * 0.3, tilesize * 0.3, 0, 0, 360);
          c2d.fill();
        }
      }
    }
  }

  /** @type {HTMLCanvasElement} */
  let canvas = (document.getElementById("thecanvas"));
  let c2d = canvas.getContext("2d");

  let gameboard = new GameBoard(8, 8);
  gameboard.addPlayer(new Player(3, 3, "red"));
  gameboard.addPlayer(new Player(3, 6, "red"));
  gameboard.addPlayer(new Player(6, 3, "blue"));
  gameboard.addPlayer(new Player(6, 6, "blue"));

  let cache = new DrawState();

  canvas.onmousemove = (function (event) {
    let tilesize = canvas.width / gameboard.width;

    let tx = event.offsetX / tilesize;
    let ty = event.offsetY / tilesize;
    if (cache.tilex != tx || cache.tiley != ty) {
      cache.tilex = tx;
      cache.tiley = ty;
      cache.redrawboard = true;
    }
  })
  animations.push((dt, totaltime) => {
    if (cache.redrawboard) {
      drawTiles(c2d, gameboard);
      cache.redrawboard = false;
    }
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
