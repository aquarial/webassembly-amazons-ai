/*jshint esversion: 6 */
// @ts-check


export class Pos {
    /**
    * @param {number} x
    * @param {number} y
    */
    constructor(y, x) {
        this.x = x
        this.y = y
    }

    copy() {
        return new Pos(this.y, this.x);
    }

    str() {
        return `${this.y} ${this.x}`
    }
}

export class Player {
    /**
     * @param {Pos} pos
     * @param {string} team
     */
    constructor(pos, team) {
        this.pos = pos
        this.team = team
    }
}

export class GameBoard {
    /**
     *
     * Board goes from (1,1) to (height,width) inclusive
     * @param {number} width
     * @param {number} height
     */
    constructor(width, height) {
        this.width = width
        this.height = height
        /** @type {Map<string,Any|Player>} */
        this.blocked = new Map()
    }

    /**
     * @param {Player|Pos} p
     */
    atPos(p) {
        if (p instanceof Player)
            p = p.pos
        return this.blocked.get(p.str())
    }
    /**
     * @param {Number} y
     * @param {Number} x
     */
    atYX(y, x) {
        return this.blocked.get(`${y} ${x}`)
    }

    /**
     * @param {Player|Pos} p0
     * @param {Player|Pos} p1
     */
    openLineTo(p0, p1) {
        if (p0 instanceof Player)
            p0 = p0.pos
        if (p1 instanceof Player)
            p1 = p1.pos
        //
        let y0 = p0.y;
        let x0 = p0.x;
        let y1 = p1.y;
        let x1 = p1.x;

        if (y0 == y1 && x0 == x1) {
            return false; // can't be same point
        }
        if (y0 != y1 && x0 != x1 && Math.abs(y1 - y0) != Math.abs(x1 - x0)) {
            return false; // not a line
        }

        let distance = Math.max(Math.abs(y1 - y0), Math.abs(x1 - x0));

        for (let dt = 1; dt <= distance; dt++) {
            if (this.atYX(y0 + dt * Math.sign(y1 - y0), x0 + dt * Math.sign(x1 - x0))) {
                return false;
            }
        }

        return true;
    }

    /**
     * @param {Player} player
     */
    addPlayer(player) {
        this.blocked.set(player.pos.str(), player)
    }

    /**
     * @param {Player} player
     * @param {Pos} move
     * @param {Pos} stone
     */
    makePlayerMove(player, move, stone) {
        this.blocked.set(player.pos.str(), undefined)
        this.blocked.set(move.str(), player)
        player.pos.y = move.y
        player.pos.x = move.x
        this.blocked.set(stone.str(), 1)
    }
}

class Move {
    constructor(player, move, stone, ai) {
        this.player = player.copy();
        this.move = move.copy();
        this.stone = stone.copy();
        this.ai = ai;
    }
}

export class GameState {

    constructor() {
        /** @type {Move[]} */
        this.history = []
        this.next_to_go = "red"
    }

    undoMove(gameboard) {
        let v = this.history.pop()

        if (v != undefined) {
            if (this.next_to_go == "red") {
                this.next_to_go = 'blue'
            } else {
                this.next_to_go = "red"
            }
            gameboard.blocked.set(v.stone.str(), undefined)

            let player = gameboard.atPos(v.move);
            gameboard.blocked.set(v.player.str(), player)
            gameboard.blocked.set(v.move.str(), undefined)
            player.pos.y = v.player.y
            player.pos.x = v.player.x
            if (v.ai) {
                this.undoMove(gameboard);
            }
        }
    }

    /**
     * @param {Player} player
     * @param {Pos} move
     * @param {Pos} stone
     */
    addMove(player, move, stone) {
        this.history.push(new Move(player.pos, move, stone, false))
        if (this.next_to_go == "red") {
            this.next_to_go = 'blue'
        } else {
            this.next_to_go = "red"
        }
    }
}
