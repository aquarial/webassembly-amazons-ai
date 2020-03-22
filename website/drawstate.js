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


  team_color(team_name, faded=false) {
    if (team_name == "block") {
      if (faded) {
        return '#30303066'
      } else {
        return '#303030'
      }

    }
    if (team_name == "red") {
      if (faded) {
        return "#ff000066"
      } else {
        return "#ff0000"
      }
    } 
    
    if (team_name == "blue") {
      if (faded) {
        return "#0000ff66"
      } else {
        return "#0000ff"
      }
    }
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

        if (this.piece == null) { // selecting a piece

          if (at instanceof Player) {
            c2d.beginPath();
            c2d.fillStyle = "white"
            c2d.ellipse((x - 1 + 0.5) * tilesize, (y - 1 + 0.5) * tilesize,
              (tilesize * 1.1) * 0.3, (tilesize * 1.1) * 0.3, 0, 0, 360);
            c2d.fill();
            c2d.fillStyle = this.team_color(at.team)
            c2d.beginPath();
            c2d.ellipse((x - 1 + 0.5) * tilesize, (y - 1 + 0.5) * tilesize,
              tilesize * 0.3, tilesize * 0.3, 0, 0, 360);
            c2d.fill();
          } else if (at != undefined) {
            c2d.fillStyle = this.team_color("block")
            c2d.fillRect((x - 1 + 0.2) * tilesize, (y - 1 + 0.2) * tilesize, tilesize * 0.6, tilesize * 0.6)
          }

        } else if (this.move == null) { // selecting a move

          if (at === this.piece) {
            c2d.beginPath();
            c2d.fillStyle = "gray"
            c2d.ellipse((x - 1 + 0.5) * tilesize, (y - 1 + 0.5) * tilesize,
              (tilesize ) * 0.3, (tilesize) * 0.3, 0, 0, 360);
            c2d.fill();
          }

          if (at == undefined && this.mouse_pos.y == y && this.mouse_pos.x == x) {
            if (gameboard.openLineTo(this.piece.pos, this.mouse_pos)) {
              c2d.fillStyle = this.team_color(this.piece.team, true)
              c2d.beginPath();
              c2d.ellipse((x - 1 + 0.5) * tilesize, (y - 1 + 0.5) * tilesize,
                tilesize * 0.3, tilesize * 0.3, 0, 0, 360);
              c2d.fill();
            }
          }

          if (at instanceof Player) {
            if (at != this.piece) {
              c2d.beginPath();
              c2d.fillStyle = "white"
              c2d.ellipse((x - 1 + 0.5) * tilesize, (y - 1 + 0.5) * tilesize,
                (tilesize * 1.1) * 0.3, (tilesize * 1.1) * 0.3, 0, 0, 360);
              c2d.fill();
              c2d.fillStyle = at.team;
              c2d.beginPath();
              c2d.ellipse((x - 1 + 0.5) * tilesize, (y - 1 + 0.5) * tilesize,
                tilesize * 0.3, tilesize * 0.3, 0, 0, 360);
              c2d.fill();
            }
          } else if (at != undefined) {
            c2d.fillStyle = this.team_color("block")
            c2d.fillRect((x - 1 + 0.2) * tilesize, (y - 1 + 0.2) * tilesize, tilesize * 0.6, tilesize * 0.6)
          }


        } else { // selecting stone

          if (at === this.piece && !(y == this.mouse_pos.y && x == this.mouse_pos.x)) {
            c2d.beginPath();
            c2d.fillStyle = "gray"
            c2d.ellipse((x - 1 + 0.5) * tilesize, (y - 1 + 0.5) * tilesize,
              (tilesize ) * 0.3, (tilesize ) * 0.3, 0, 0, 360);
            c2d.fill();
          }

          if (at instanceof Player) {
            if (at != this.piece) {
              c2d.beginPath();
              c2d.fillStyle = "white"
              c2d.ellipse((x - 1 + 0.5) * tilesize, (y - 1 + 0.5) * tilesize,
                (tilesize * 1.1) * 0.3, (tilesize * 1.1) * 0.3, 0, 0, 360);
              c2d.fill();
              c2d.fillStyle = this.team_color(at.team)
              c2d.beginPath();
              c2d.ellipse((x - 1 + 0.5) * tilesize, (y - 1 + 0.5) * tilesize,
                tilesize * 0.3, tilesize * 0.3, 0, 0, 360);
              c2d.fill();
            }
          } else if (at != undefined) {
            c2d.fillStyle = this.team_color("block")
            c2d.fillRect((x - 1 + 0.2) * tilesize, (y - 1 + 0.2) * tilesize, tilesize * 0.6, tilesize * 0.6)
          } else if (y == this.move.y && x == this.move.x) {
            c2d.fillStyle = this.team_color(this.piece.team)
            c2d.beginPath();
            c2d.ellipse((x - 1 + 0.5) * tilesize, (y - 1 + 0.5) * tilesize,
              tilesize * 0.3, tilesize * 0.3, 0, 0, 360);
            c2d.fill();
          }

          gameboard.blocked.set(this.piece.pos.str(), undefined);
          if (y == this.mouse_pos.y && x == this.mouse_pos.x) {
            if (gameboard.openLineTo(this.move, this.mouse_pos)) {
              c2d.fillStyle = this.team_color("block", true)
              c2d.fillRect((x - 1 + 0.2) * tilesize, (y - 1 + 0.2) * tilesize, tilesize * 0.6, tilesize * 0.6)
            }
          }
          gameboard.blocked.set(this.piece.pos.str(), this.piece);


        }
      }
    }
  }
}
