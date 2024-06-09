import * as PIXI from 'pixi.js';
import { Game } from './game';

let type = "WebGL";
if(!PIXI.utils.isWebGLSupported()){
    type = "canvas";
}
PIXI.utils.sayHello(type);

window.onload = start;


async function start() {
    let game = new Game();
    window.game = game;
    game.start();

    displayVersion();
}

function displayVersion() {
    const versionText = new PIXI.Text(`Version: ${process.env.GAME_VERSION}`, { fontSize: 12, fill: 'black' });
    versionText.x = 10;
    versionText.y = window.innerHeight - 20;
    window.game.app.stage.addChild(versionText);
}
