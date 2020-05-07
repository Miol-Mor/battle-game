let type = "WebGL"
if(!PIXI.utils.isWebGLSupported()){
    type = "canvas"
}

PIXI.utils.sayHello(type)

window.onload = start;

function start() {
    //Create a Pixi Application
    let app = new PIXI.Application({ 
        width: 256,         // default: 800
        height: 256,        // default: 600
        antialias: true,    // default: false
        transparent: false, // default: false
        resolution: 1       // default: 1
        }
    );
    app.renderer.backgroundColor = 0xd6b609;
    // console.log(app);
    // console.log(app.renderer.view.width, app.renderer.view.height);

    app.renderer.autoDensity = true; // autoResize is deprecated
    app.renderer.resize(window.innerWidth, window.innerHeight);

    //Add the canvas that Pixi automatically created for you to the HTML document
    document.body.appendChild(app.view); // app.view - canvas element

    // console.log(app.stage);
    
    // PIXI.loader is deprecated
    // const loader = PIXI.Loader.shared; // PixiJS exposes a premade instance for you to use.
    app.loader.add('red unit', 'images/red unit.png');
    app.loader.add('blue unit', 'images/blue unit.png');
    app.loader.add('kitten', 'https://ae01.alicdn.com/kf/HTB1zD1Re4SYBuNjSspjq6x73VXaT/Plastic-Puzzles-1000-pieces-Different-Shapes-Cute-Cat-Animal-jigsaws-puzzle-with-gift-box-for-Adult.jpg');
    app.loader.load(() => {setup(app)});
}

function setup(app) {
    // console.log(PIXI.utils.TextureCache);
    // console.log(app.loader.resources['blue unit']);
    let red_unit = new PIXI.Sprite(
        app.loader.resources["red unit"].texture
    );
    let blue_unit = new PIXI.Sprite(
        app.loader.resources["blue unit"].texture
    );
    let fighters = new PIXI.Container();
    fighters.addChild(red_unit);
    fighters.addChild(blue_unit);
    // console.log(fighters.children[0] == red_unit); // returns true
    // app.stage.addChild(red_unit);
    // app.stage.addChild(blue_unit);
    app.stage.addChild(fighters);
    blue_unit.x = 200;
    red_unit.position.set(100, 100);
    // red_unit.width = 80;
    red_unit.height = 80;
    blue_unit.scale.set(1.2, 1.2);
    red_unit.anchor.set(0.5, 0.5);
    red_unit.rotation = Math.PI * 1.5;
    units = [red_unit, blue_unit];
    // app.stage.removeChild(blue_unit);
    // blue_unit.visible = false;
    // blue_unit.texture = app.loader.resources['kitten'].texture;

    let message = new PIXI.Text("Hello Pixi!");
    app.stage.addChild(message);
    message.position.set(200, 60);
    message.alpha = 0.5;
    message.style = {dropShadow: true};
    message.text = "Hi, guys!";

    // Hex
    let hex = draw_hex(app);
    app.stage.addChild(hex);


    b = new Bump(PIXI);
    
    game_objs = {};
    game_objs.states = {};
    game_objs.states.play = play;
    game_objs.states.reverse = reverse;
    game_objs.units = units;
    game_objs.message = message;
    state = play;
    app.ticker.add(delta => gameLoop(delta, game_objs, app.ticker));
    // console.log(app.ticker);
}

function draw_hex(app) {
    let hex = new PIXI.Graphics();
    let r = 50;
    // hex.beginFill(0x66FF33);

    //Use `drawPolygon` to define the triangle as
    //a path array of x/y positions
    hex.lineStyle(1, 0x000000, 1);

    hex.drawPolygon([
        - r * Math.sqrt(3) / 2, r / 2,
        0, r,
        r * Math.sqrt(3) / 2, r / 2,
        r * Math.sqrt(3) / 2, - r / 2,
        0, - r,
        - r * Math.sqrt(3) / 2, - r / 2
    ]);

    //Fill shape's color
    // hex.endFill();

    //Position the triangle after you've drawn it.
    //The triangle's x/y position is anchored to its first point in the path
    hex.x = 300;
    hex.y = 300;

    return hex;
}

function play(delta, units) {
    red_unit = units[0];
    blue_unit = units[1];
    blue_unit.x += 1;
    red_unit.rotation += delta / 100;
}

function reverse(delta, units) {
    red_unit = units[0];
    blue_unit = units[1];
    blue_unit.x -= 1;
    red_unit.rotation -= delta / 100;
}

function gameLoop(delta, game_objs, ticker) {
    if ((Math.floor(ticker.lastTime / 7000) % 2) == 0) {
        state = game_objs.states.reverse;
    }
    else {
        state = game_objs.states.play;
    }
    if (b.hitTestRectangle(game_objs.units[0], game_objs.units[1])) {
        game_objs.message.text = "Collision!";
    }
    else {
        game_objs.message.text = "No collision";
    }
    state(delta, game_objs.units);
}