import * as PIXI from 'pixi.js';
import { Hex_grid } from './grid';
import { Unit } from './unit';

export class Game {
    // players_num - number of players (for the future)
    // app - PIXI aplication, used for this game
    // grid - grid for this game (Hex_grid)
    constructor(players_num = 2) {
        this.players_num = players_num;
        this.app = null;
        this.grid = null;

        // graphic constants
        this.BACKGROUND_COLOR = 0xd6b609;
    }

    // needed, because constructor cannot be async
    async start() {
        // correct order of functions is important
        this.create_stage();
        await this.load_images();
        let data = await this.read_socket();
        let field_data = JSON.parse(data);
        this.create_grid(field_data);
        this.set_units_start_pos(field_data);
    }

    // private
    create_stage() {
        let app = new PIXI.Application({
            antialias: true,
            transparent: false,
            resolution: 1
        });
        app.renderer.backgroundColor = this.BACKGROUND_COLOR;

        // resize PIXI canvas acording to size of window
        app.renderer.autoDensity = true;
        app.renderer.resize(window.innerWidth, window.innerHeight);

        // add the canvas that Pixi automatically created for you to the HTML document
        document.body.appendChild(app.view); // app.view - canvas element

        this.app = app;
    }

    // private
    load_images() {
        this.app.loader.add('red unit', 'images/red unit.png');
        this.app.loader.add('blue unit', 'images/blue unit.png');

        return new Promise(resolve => {
            this.app.loader.load(function() {
                resolve("images loaded");
            });
        });
    }

    // private
    // wait for message recieved from server, then go on
    read_socket() {
        let socket = new WebSocket("ws://127.0.0.1:8088/ws/");

        socket.onopen = function(e) {
            console.log("[open] Connection established");
        };

        let p = new Promise(resolve => {
            socket.onmessage = function(event) {
                console.log(`[message] Data received from server: ${event.data}`);
                resolve(event.data);
            };
        });

        socket.onclose = function(event) {
            if (event.wasClean) {
                console.log(`[close] Connection closed cleanly, code=${event.code} reason=${event.reason}`);
            } else {
                // e.g. server process killed or network down
                // event.code is usually 1006 in this case
                console.log('[close] Connection died');
            }
        };

        socket.onerror = function(error) {
            console.log(`[error] ${error.message}`);
        };

        return p;
    }

    // private
    create_grid(field_data) {
        let grid = new Hex_grid(this.app.stage, field_data.row_n, field_data.col_n, 50, 100, 100);
        grid.draw();

        let hexes = field_data.field.hexes;
        for (let i = 0; i < field_data.row_n * field_data.col_n; i++) {
            if (hexes[i].content != undefined) {
                if (hexes[i].content.type == 'wall') {
                    grid.fill_hex(hexes[i].y, hexes[i].x);
                }
            }
        }

        this.grid = grid;
    }

    // private
    set_units_start_pos(field_data) {
        let hexes = field_data.field.hexes;
        for (let i = 0; i < field_data.row_n * field_data.col_n; i++) {
            if (hexes[i].unit != undefined) {
                let texture;
                if (hexes[i].unit.player == 1) {
                    texture = this.app.loader.resources["blue unit"].texture;
                } else {
                    texture = this.app.loader.resources["red unit"].texture;
                }
                this.create_unit(texture, this.grid.hex_size, hexes[i].unit, hexes[i].y, hexes[i].x);
            }
        }
    }

    // actions with units
    create_unit(texture, img_size, params, y, x) {
        let unit = new Unit(texture, img_size, params);
        this.grid.hexes[y][x].set_unit(unit);
        return unit;
    }

    move_unit(from_y, from_x, to_y, to_x) {
        let from_hex = this.grid.hexes[from_y][from_x];
        let to_hex = this.grid.hexes[to_y][to_x];
        let unit = from_hex.unit;

        from_hex.unset_unit();
        to_hex.set_unit(unit);
    }
}
