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
   * @param {number} tilesize
   * @param {number} y
   * @param {number} x
   * @param {String} style
   */
  drawOneShape(c2d, tilesize, y, x, style) {
    switch (style) {
      case "circle":
        c2d.beginPath();
        c2d.ellipse((x - 1 + 0.5) * tilesize, (y - 1 + 0.5) * tilesize,
          (tilesize) * 0.3, (tilesize) * 0.3, 0, 0, 360);
        c2d.fill();
        break;
      case "circle outline":
        c2d.beginPath();
        c2d.ellipse((x - 1 + 0.5) * tilesize, (y - 1 + 0.5) * tilesize,
          (tilesize * 1.1) * 0.3, (tilesize * 1.1) * 0.3, 0, 0, 360);
        c2d.fill();
        break;
      case "block":
        c2d.fillRect((x - 1 + 0.2) * tilesize, (y - 1 + 0.2) * tilesize, tilesize * 0.6, tilesize * 0.6)
        break;
    }
  }

  /**
   * @param {string} team_name
   */
  team_color(team_name, faded=false) {
    if (team_name == "deselected") {
      return "#30303055"
    }
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
    throw new Error("Unkown team: " + team_name)
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
            c2d.fillStyle = "white"
            this.drawOneShape(c2d, tilesize, y, x, "circle outline")
            c2d.fillStyle = this.team_color(at.team)
            this.drawOneShape(c2d, tilesize, y, x, "circle")
          } else if (at != undefined) {
            c2d.fillStyle = this.team_color("block")
            this.drawOneShape(c2d, tilesize, y, x, "block")
          }

        } else if (this.move == null) { // selecting a move

          if (at === this.piece) {
            c2d.fillStyle = this.team_color("deselected")
            this.drawOneShape(c2d, tilesize, y, x, "circle")
          }

          if (at == undefined && this.mouse_pos.y == y && this.mouse_pos.x == x) {
            if (gameboard.openLineTo(this.piece.pos, this.mouse_pos)) {
              c2d.fillStyle = this.team_color(this.piece.team, true)
              this.drawOneShape(c2d, tilesize, y, x, "circle")
            }
          }

          if (at instanceof Player) {
            if (at != this.piece) {
              c2d.fillStyle = "white"
              this.drawOneShape(c2d, tilesize, y, x, "circle outline")
              c2d.fillStyle = this.team_color(at.team)
              this.drawOneShape(c2d, tilesize, y, x, "circle")  
            }
          } else if (at != undefined) {
            c2d.fillStyle = this.team_color("block")
            this.drawOneShape(c2d, tilesize, y, x, "block")
          }


        } else { // selecting stone

          if (at === this.piece && !(y == this.mouse_pos.y && x == this.mouse_pos.x)) {
            c2d.fillStyle = this.team_color("deselected")
            this.drawOneShape(c2d, tilesize, y, x, "circle")
          }

          if (at instanceof Player) {
            if (at != this.piece) {
              c2d.fillStyle = "white"
              this.drawOneShape(c2d, tilesize, y, x, "circle outline")
              c2d.fillStyle = this.team_color(at.team)
              this.drawOneShape(c2d, tilesize, y, x, "circle")
            }
          } else if (at != undefined) {
            c2d.fillStyle = this.team_color("block")
            this.drawOneShape(c2d, tilesize, y, x, "block")
          } else if (y == this.move.y && x == this.move.x) {
            c2d.fillStyle = this.team_color(this.piece.team)
            this.drawOneShape(c2d, tilesize, y, x, "circle")
        }

          gameboard.blocked.set(this.piece.pos.str(), undefined);
          if (y == this.mouse_pos.y && x == this.mouse_pos.x) {
            if (gameboard.openLineTo(this.move, this.mouse_pos)) {
              c2d.fillStyle = this.team_color("block", true)
              this.drawOneShape(c2d, tilesize, y, x, "block")
            }
          }
          gameboard.blocked.set(this.piece.pos.str(), this.piece);


        }
      }
    }
  }
}
