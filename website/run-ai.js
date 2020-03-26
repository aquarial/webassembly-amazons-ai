// @ts-check
/* jshint -W069, esversion:6 */

onmessage = (ev) => {
    console.log("run-ai.js received", ev);

    if ('wasm' in ev.data) {
        postMessage("procceseeeeeeeeeee")

        let board = ev.data["wasm"]
        console.log("Got " , ev.data, ev.data.wasm)
        console.log("Valied:", board.is_valid);
        board.free();
    }

    // if (board.is_valid()) {
    //     let r = wasm.compute_ai_move(board)
    //     console.log(r.piece_y, r.piece_x, r.move_y, r.move_x, r.stone_y, r.stone_x)
    //     if (r.piece_y > 0 && r.piece_x > 0 &&
    //         r.move_y > 0 && r.move_x > 0 &&
    //         r.stone_y > 0 && r.stone_x > 0) {
    //         let p0 = gameboard.atPos(new Pos(r.piece_y, r.piece_x))
    //         let p1 = new Pos(r.move_y, r.move_x)
    //         let p2 = new Pos(r.stone_y, r.stone_x)
    //         gamestate.addMove(p0, p1, p2)
    //         gameboard.makePlayerMove(p0, p1, p2)
    //         drawstate.piece = null
    //         drawstate.move = null;
    //     }
    //     r.free();
    // } else {
    //     alert("Error: invalid board??")
    // }

    // board.free();
}

