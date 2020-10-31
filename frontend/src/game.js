import * as PIXI from 'pixi.js';
import { Hex_grid } from './grid';
import { Unit } from './unit';


export class Game {
    constructor() {
        // Websocket we use to contact with server
        this.socket = null;
        // app - PIXI aplication, used for this game
        this.app = null;

        // array of game states
        this.STATES = {
            MOVE_FROM: 'move_from',
            MOVE_TO: 'move_to',
            ATTACK: 'attack',
            WAIT: 'wait',
            END: 'end'
        };

        // graphic constants
        this.BACKGROUND_COLOR = 0xd6b609;

        // set map of commands
        this.cmd_map = {};
        // reset game (or create a new one if doesn't exist)
        this.cmd_map.field = this.create_new_field;

        this.cmd_map.turn = function(data) {
            this.check_state(this.STATES.WAIT);
            this.change_state(this.STATES.MOVE_FROM);
            this.show_tooltip();
        };

        this.cmd_map.moving = function(data) {
            this.check_state(this.STATES.MOVE_TO, this.STATES.WAIT);
            data.type = 'move';
            this.redraw_field(data);
            if (this.state === this.STATES.MOVE_TO) {
                this.change_state(this.STATES.ATTACK);
            }
            this.show_tooltip();
        };

        this.cmd_map.attacking = function(data) {
            this.check_state(this.STATES.ATTACK, this.STATES.WAIT);
            data.type = 'attack';
            this.redraw_field(data);
            if (this.state === this.STATES.ATTACK) {
                this.change_state(this.STATES.WAIT);
                let hex = this.grid.hexes[this.cur_move.to.x][this.cur_move.to.y];
                if (hex.unit) {
                    hex.unit.stop_pulse();
                }
            }
            this.show_tooltip();
        };

        this.cmd_map.error = function(data) {
            console.error(JSON.stringify(data));
        };

        this.cmd_map['GFY! :D'] = function(data) {
            alert('GFY! :D');
        };
    }

    // (needed, because constructor cannot be async)
    async start() {
        this.create_stage();
        await this.load_images();
        this.create_socket();
    }

    // create websocket to receive and send messages
    // private
    create_socket() {
        this.socket = new WebSocket("ws://127.0.0.1:8088/ws/");

        this.socket.onopen = function(e) {
            console.log("[open] Connection established");
        };

        this.socket.onmessage = this.process_message.bind(this);

        this.socket.onclose = function(event) {
            if (event.wasClean) {
                console.log(`[close] Connection closed cleanly, code=${event.code} reason=${event.reason}`);
            } else {
                // e.g. server process killed or network down
                // event.code is usually 1006 in this case
                console.log('[close] Connection died');
            }
        };

        this.socket.onerror = function(error) {
            console.log(`[error] ${error.message}`);
        };
    }


    // States operations
    // check there is correct state
    // private
    check_state(...states) {
        if (! states.includes(this.state)) {
            throw new Error(`Incorrect game state: ${this.state}; ${states} expected`);
        }
    }

    //private
    change_state(state) {
        this.state = state;
    }


    // Game cycle
    // process messages from the server
    // private
    process_message(event) {
        let data = JSON.parse(event.data);
        console.log(data);
        // TODO: we need to wait here all previous actions done
        this.cmd_map[data.cmd].call(this, data);
    }

    // process users clicks
    // private
    process_click(event) {
        console.log('clicked', event.target.coords);
        switch (this.state) {
            case this.STATES.MOVE_FROM:
                this.move_from(event.target.coords);
                this.change_state('move_to');

                let hex = this.grid.hexes[event.target.coords.x][event.target.coords.y];
                if (hex.unit) {
                    hex.unit.start_pulse();
                }
            break;

            case this.STATES.MOVE_TO:
                this.move_to(event.target.coords);
                this.send_move();
            break;

            case this.STATES.ATTACK:
                this.attack(event.target.coords);
                this.send_attack();
            break;

            default:
                console.log('default click');
            break;
        }
        this.show_tooltip();
    }


    // Create new field functions
    // private
    create_new_field(field_data) {
        // correct order of functions is important
        console.log("creating new field");
        this.set_defaults();
        this.clear_app();
        this.create_grid(field_data);
        this.set_units_start_pos(field_data);
        this.create_info();
        this.set_hex_click_handlers();
    }

    // set start values of game fields
    set_defaults() {
        // grid - grid for this game (Hex_grid)
        this.grid = null;

        // players_num - number of players (for the future)
        this.players_num = 2;
        // my_num - number of player using this client
        this.my_num = 1;

        this.cur_hex = null;
    }

    // remove everything from the PIXI stage
    clear_app() {
        this.app.stage.removeChildren();
    }

    // private
    create_stage() {
        let app = new PIXI.Application ({
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
    create_grid(field_data) {
        let grid = new Hex_grid(this.app.stage, field_data.num_x, field_data.num_y, 50, 100, 100);
        grid.draw();

        let hexes = field_data.field.hexes;
        for (let i = 0; i < field_data.num_x * field_data.num_y; i++) {
            if (hexes[i].content && hexes[i].content.type === 'wall') {
                grid.hexes[hexes[i].x][hexes[i].y].set_content('wall');
            }
        }

        this.grid = grid;
    }

    // private
    set_units_start_pos(field_data) {
        let hexes = field_data.field.hexes;
        for (let i = 0; i < field_data.num_x * field_data.num_y; i++) {
            if (hexes[i].unit !== undefined) {
                let texture;
                if (hexes[i].unit.player === 1) {
                    texture = this.app.loader.resources["blue unit"].texture;
                } else {
                    texture = this.app.loader.resources["red unit"].texture;
                }
                this.create_unit(texture, this.grid.hex_size, hexes[i].unit, hexes[i].x, hexes[i].y);
            }
        }
    }

    // private
    create_info() {
        this.info = new PIXI.Text();
        this.app.stage.addChild(this.info);
        this.clear_info();
    }

    // private
    clear_info() {
        this.info.text = this.turn_info();
        this.info.text += 'Info:\n';
    }

    // private
    set_info(params) {
        this.clear_info();
        for (let [key, value] of Object.entries(params)) {
            this.info.text += `${key}: ${value}\n`;
        }
    }

    // private
    turn_info() {
        switch (this.state) {
            case this.STATES.WAIT:
                return 'Wait for opponents turn\n';
            case this.STATES.MOVE_FROM:
                return 'Your turn: Select unit\n';
            case this.STATES.MOVE_TO:
                return 'Your turn: Move unit\n';
            case this.STATES.ATTACK:
                return 'Your turn: Attack\n';
            default:
                return 'Just wait a second\n';
        }
    }

    // private
    set_hex_click_handlers() {
        for (let x = 0; x < this.grid.num_x; x++) {
            for (let y = 0; y < this.grid.num_y; y++) {
                let hex = this.grid.hexes[x][y];
                hex.interactive = true;
                hex.hitArea = hex.polygon;
                hex.on('click', this.process_click.bind(this));
                hex.on('mouseover', this.process_mouseover.bind(this));
                hex.on('mouseout', this.process_mouseout.bind(this));
            }
        }
    }

    // private
    process_mouseover(event) {
        this.cur_hex = event.currentTarget;
        event.currentTarget.highlight();
        this.show_tooltip(event.currentTarget);
    }

    // private
    show_tooltip() {
        // if player hovers on hex with unit, show info about it
        if (this.cur_hex && this.cur_hex.unit) {
            this.set_info(this.cur_hex.unit.params);
        }
        // else just show ordinary info
        else if (this.info) { // it's a crutch, because we receive 'turn' message before 'info' block created
            this.clear_info();
        }
    }

    // private
    process_mouseout(event) {
        event.currentTarget.dim();
        this.clear_info();
        this.cur_hex = null;
    }

    send_to_backend(target) {
        this.socket.send (
            JSON.stringify ({
                "cmd": "click",
                "target": target,
            })
        );
    }

    // Change field functions
    // private
    redraw_field(data) {
        console.log('redraw field');
        switch(data.type) {
            case 'move':
                this.move_unit(data.coords[0].x, data.coords[0].y, data.coords[1].x, data.coords[1].y);
            break;

            case 'attack':
                console.log('attack!!! charge!!!');
                // animate attack
                if (data.changes) {
                    if (data.changes.hurt) {
                        data.changes.hurt.forEach(el => {
                            this.change_hex(el);
                        });
                    }
                    if (data.changes.die) {
                        data.changes.die.forEach(el => {
                            this.kill_unit(el);
                        });
                    }
                }
            break;
        }
    }

    // private
    change_hex(hex_data) {
        console.log(hex_data);
        let hex = this.grid.hexes[hex_data.x][hex_data.y];
        if (hex_data.content) {
            hex.set_content(hex_data.content);
        }

        if (hex_data.unit) {
            hex.change_unit(hex_data.unit);
        }
    }


    // Actions with units
    // private
    create_unit(texture, img_size, params, x, y) {
        let unit = new Unit(texture, img_size, params);
        this.grid.hexes[x][y].set_unit(unit);

        this.app.ticker.add(() => {
            unit.pulse();
        });

        return unit;
    }

    // private
    move_unit(from_y, from_x, to_y, to_x) {
        let from_hex = this.grid.hexes[from_y][from_x];
        let to_hex = this.grid.hexes[to_y][to_x];
        let unit = from_hex.unit;
        if (from_hex.unit === null) {
            throw new Error(`No unit in the cell to move from: ${JSON.stringify(from_hex.coords)}`);
        }
        if (to_hex.unit !== null) {
            throw new Error(`Cell to move to is already occupied: ${JSON.stringify(to_hex.coords)}`);
        }

        from_hex.unset_unit();
        to_hex.set_unit(unit);
    }

    // private
    kill_unit(hex_data) {
        let hex = this.grid.hexes[hex_data.x][hex_data.y];
        hex.erase_unit();
        hex.unset_unit();
    }

    // dev
    // private
    find_unit(player) {
        for (let x = 0; x < this.grid.num_x; x++) {
            for (let y = 0; y < this.grid.num_y; y++) {
                let unit = this.grid.hexes[x][y].unit;
                if (unit && unit.params.player === player) {
                    return this.grid.hexes[x][y].coords;
                }
            }
        }
        return null;
    }

    // dev
    // private
    find_empty_hex() {
        for (let x = 0; x < this.grid.num_x; x++) {
            for (let y = 0; y < this.grid.num_y; y++) {
                let hex = this.grid.hexes[x][y];
                if (! hex.content && ! hex.unit) {
                    return hex.coords;
                }
            }
        }
        return null;
    }
}
