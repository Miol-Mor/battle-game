let type = "WebGL";
if(!PIXI.utils.isWebGLSupported()){
    type = "canvas";
}
PIXI.utils.sayHello(type);

window.onload = start;

async function start() {
    game = new Game();
    await game.start();

    // inspect all our units
    for (let y = 0; y < game.grid.row_n; y++) {
        for (let x = 0; x < game.grid.col_n; x++) {
            if (game.grid.hexs[y][x].unit) {
                console.log(y, x);
                console.log(game.grid.hexs[y][x].unit.params);
            }
        }
    }

    game.move_unit(1, 3, 5, 0);
}
