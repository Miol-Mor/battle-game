import * as PIXI from 'pixi.js';
import { Hex_grid } from './grid';
import { Unit } from './unit';

export class Game {
    // players_num - number of players (for the future)
    // app - PIXI aplication, used for this game
    // grid - grid for this game (Hex_grid)
    constructor(players_num = 2, grid_params = {}) {
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
        this.create_grid();
        this.set_units_start_pos();
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
    create_grid() {
        let grid = new Hex_grid(this.app.stage, 6, 8, 50, 100, 100);
        grid.draw();

        grid.fill_hex(2, 5);

        this.grid = grid;
    }

    // private
    set_units_start_pos() {
        let blue_unit = this.create_unit(this.app.loader.resources["blue unit"].texture, this.grid.hex_size,
            {HP: 10, damage: 3}, 1, 3);
        let red_unit = this.create_unit(this.app.loader.resources["red unit"].texture, this.grid.hex_size,
            {HP: 5, damage: 4}, 2, 0);
        // let red_unit2 = new Unit(this.app.loader.resources["red unit"].texture, this.grid, 4, 5, {HP: 7, damage: 2});
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
