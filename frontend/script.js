let type = "WebGL"
if(!PIXI.utils.isWebGLSupported()){
    type = "canvas"
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

    app.renderer.autoDensity = true;
    app.renderer.resize(window.innerWidth, window.innerHeight);

    //Add the canvas that Pixi automatically created for you to the HTML document
    document.body.appendChild(app.view); // app.view - canvas element

    grid = new Hex_grid(app.stage, 6, 8, 50, 100, 100);
    grid.draw();

    grid.fill_hex(2, 5);
}
