/*jshint esversion: 6 */
// @ts-check

import { Player, Pos, GameBoard } from "./game";



export class DrawState {
  constructor() {
    this.mouse_pos = new Pos(-1,-1);

    /** @type {Player} */
    this.piece = null

    /** @type {Pos} */
    this.move = null
  }


  /**
   * @param {CanvasRenderingContext2D} c2d
   * @param {GameBoard} gameboard
   * @param {number} tilesize
   */
  drawTiles(c2d, gameboard, tilesize) {
    let checker_colors = ["#eae8ea", "#c1c1c1"]

    for (let y = 1; y <= gameboard.height; y++) {
      for (let x = 1; x <= gameboard.width; x++) {
        c2d.fillStyle = checker_colors[(x + y) % 2]
        c2d.fillRect((x - 1) * tilesize, (y - 1) * tilesize, tilesize, tilesize)

        let at = gameboard.atYX(y, x);
        if (at instanceof Player) {
          if (at == this.piece) {
            c2d.beginPath();
            c2d.fillStyle = "gray"
            c2d.ellipse((x - 1 + 0.5) * tilesize, (y - 1 + 0.5) * tilesize,
              (tilesize * 1.4) * 0.3, (tilesize * 1.4) * 0.3, 0, 0, 360);
            c2d.fill();
          }
          c2d.beginPath();
          c2d.fillStyle = "white"
          c2d.ellipse((x - 1 + 0.5) * tilesize, (y - 1 + 0.5) * tilesize,
            (tilesize * 1.1) * 0.3, (tilesize * 1.1) * 0.3, 0, 0, 360);
          c2d.fill();
          c2d.fillStyle = at.team
          c2d.beginPath();
          c2d.ellipse((x - 1 + 0.5) * tilesize, (y - 1 + 0.5) * tilesize,
            tilesize * 0.3, tilesize * 0.3, 0, 0, 360);
          c2d.fill();
        } else if (at != undefined) {
          c2d.fillStyle = '#303030'
          c2d.fillRect((x - 1 + 0.2) * tilesize, (y - 1 + 0.2) * tilesize, tilesize * 0.6, tilesize * 0.6)
        }
      }
    }
  }
}
