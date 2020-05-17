let type = "WebGL";
if(!PIXI.utils.isWebGLSupported()){
    type = "canvas";
}
PIXI.utils.sayHello(type);

window.onload = start;

function start() {
    let app = new PIXI.Application({
        width: 256,
        height: 256,
        antialias: true,
        transparent: false,
        resolution: 1
    });
    app.renderer.backgroundColor = 0xd6b609;

    // resize PIXI canvas acording to size of window
    app.renderer.autoDensity = true;
    app.renderer.resize(window.innerWidth, window.innerHeight);

    // add the canvas that Pixi automatically created for you to the HTML document
    document.body.appendChild(app.view); // app.view - canvas element

    // load images
    app.loader.add('red unit', 'images/red unit.png');
    app.loader.add('blue unit', 'images/blue unit.png');
    app.loader.load(() => {loaded(app)});
}

function loaded(app) {
    grid = new Hex_grid(app.stage, 6, 8, 50, 100, 100);
    grid.draw();

    grid.fill_hex(2, 5);

    blue_unit = new Unit(app.loader.resources["blue unit"].texture, grid, 1, 3, {HP: 10, damage: 3});
    red_unit = new Unit(app.loader.resources["red unit"].texture, grid, 2, 0, {HP: 5, damage: 4});
    red_unit2 = new Unit(app.loader.resources["red unit"].texture, grid, 4, 5, {HP: 7, damage: 2});
    // blue_unit2 = new Unit(app.loader.resources["blue unit"].texture, grid, 4, 5, {HP: 10, damage: 3});

    blue_unit.move_to(4, 0);
    red_unit2.move_to(3, 5);
    // grid.hexs[0][0].addChild(blue_unit.sprite)

    for (let y = 0; y < grid.row_n; y++) {
        for (let x = 0; x < grid.col_n; x++) {
            if (grid.hexs[y][x].unit) {
                console.log(y, x);
                console.log(grid.hexs[y][x].unit.params);
            }
        }
    }
}

