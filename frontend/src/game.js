import * as PIXI from 'pixi.js';
import { Hex_grid } from './grid';
import { Unit } from './unit';


export class Game {
    constructor() {
        // app - PIXI aplication, used for this game
        this.app = null;
        // grid - grid for this game (Hex_grid)
        this.grid = null;

        // Websocket we use to contact with server
        this.socket = null;

        // players_num - number of players (for the future)
        this.players_num = 2;
        // my_num - number of player using this client
        this.my_num = 1;

        // array of game states
        this.STATES = {
            PREPARE: 'prepare',
            MOVE_FROM: 'move_from',
            MOVE_TO: 'move_to',
            ATTACK: 'attack',
            WAIT: 'wait',
            END: 'end'
        };
        this.state = this.STATES.PREPARE;
        // current player's move and attack
        this.cur_move = {from: null, to: null};
        this.cur_attack = null;

        // graphic constants
        this.BACKGROUND_COLOR = 0xd6b609;
    }

    // (needed, because constructor cannot be async)
    start() {
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
    process_message() {
        let data = JSON.parse(event.data);
        console.log(data);
        // TODO: we need to wait here all previous actions done
        switch (data.cmd) {
            case 'field':
                this.check_state(this.STATES.PREPARE);
                this.create_field(data);
                this.change_state(this.STATES.WAIT);
            break;

            case 'turn':
                this.check_state(this.STATES.WAIT);
                this.change_state(this.STATES.MOVE_FROM);
            break;

            case 'change':
                this.check_state(this.STATES.MOVE_TO, this.STATES.ATTACK, this.STATES.WAIT);
                this.redraw_field(data);
                switch (this.state) {
                    case this.STATES.MOVE_TO:
                        this.change_state(this.STATES.ATTACK);
                    break;

                    case this.STATES.ATTACK:
                        this.change_state(this.STATES.WAIT);

                        // and more simulation of server activity
                        this.socket.send (
                            JSON.stringify ({
                                "cmd": "change",
                                "type": "move",
                                "coords": [
                                    this.cur_move.to,
                                    {y: 0, x: 0}
                                ]
                            })
                        );

                        this.socket.send (
                            JSON.stringify ({
                                "cmd": "change",
                                "type": "attack",
                                "coords": {
                                    "from": this.cur_move.to,
                                    "to": this.cur_attack
                                },
                                "changes": null
                            })
                        );

                        this.socket.send(`{"cmd": "turn"}`);
                    break;
                }
            break;

            // just a crutch
            // my message is better
            default:
                console.log('default message');
                this.socket.send (
                    `{"cmd": "field", "row_n":2,"col_n":2,"field":{"hexes":[
                        {"x":0,"y":0,"unit":{"player":1,"hp":1,"attack":[1,2],"speed":1}},
                        {"x":0,"y":1},
                        {"x":1,"y":0},
                        {"x":1,"y":1,"content":{"type":"wall"}}
                    ]}}`);
            break;
        }
    }

    // process users clicks
    // private
    process_click(event) {
        console.log('clicked', event.target.coords);
        switch (this.state) {
            case this.STATES.MOVE_FROM:
                this.move_from(event.target.coords);
                this.change_state('move_to');
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
    }


    // Create field functions
    // private
    async create_field(field_data) {
        // correct order of functions is important
        this.create_stage();
        await this.load_images();
        this.create_grid(field_data);
        this.set_units_start_pos(field_data);
        this.create_info();
        this.set_hex_click_handlers();

        // simulate server activity
        this.socket.send(`{"cmd": "turn"}`);
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
        let grid = new Hex_grid(this.app.stage, field_data.row_n, field_data.col_n, 50, 100, 100);
        grid.draw();

        let hexes = field_data.field.hexes;
        for (let i = 0; i < field_data.row_n * field_data.col_n; i++) {
            if (hexes[i].content !== undefined) {
                if (hexes[i].content.type === 'wall') {
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
            if (hexes[i].unit !== undefined) {
                let texture;
                if (hexes[i].unit.player === 1) {
                    texture = this.app.loader.resources["blue unit"].texture;
                } else {
                    texture = this.app.loader.resources["red unit"].texture;
                }
                this.create_unit(texture, this.grid.hex_size, hexes[i].unit, hexes[i].y, hexes[i].x);
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
        this.info.text = 'Info:\n';
    }

    set_info(params) {
        this.clear_info();
        for (let [key, value] of Object.entries(params)) {
            this.info.text += `${key}: ${value}\n`;
        }
    }

    // private
    set_hex_click_handlers() {
        for (let y = 0; y < this.grid.row_n; y++) {
            for (let x = 0; x < this.grid.col_n; x++) {
                let hex = this.grid.hexes[y][x];
                hex.interactive = true;
                hex.hitArea = hex.polygon;
                hex.on('click', this.process_click.bind(this));
                hex.on('mouseover', this.show_tooltip.bind(this));
            }
        }
    }

    // private
    show_tooltip(event) {
        if (event.target.unit !== null) {
            this.set_info(event.target.unit.params);
        }
        else {
            this.clear_info();
        }
    }


    // Users click handlers
    // private
    move_from(coords) {
        this.cur_move.from = coords;
    }

    // private
    move_to(coords) {
        this.cur_move.to = coords;
    }

    // private
    attack(coords) {
        this.cur_attack = coords;
    }


    // Send messages to server
    // ideally send current players move to server
    // but instead we will simulate server activity
    // private
    send_move() {
        this.socket.send (
            JSON.stringify ({
                "cmd": "change",
                "type": "move",
                "coords": [
                    this.cur_move.from,
                    this.cur_move.to
                ]
            })
        );
    }

    // private
    send_attack() {
        this.socket.send(
            JSON.stringify({
                "cmd": "change",
                "type": "attack",
                "coords": {
                    "from": this.cur_move.to,
                    "to": this.cur_attack
                },
                "changes": null
            })
        );
    }


    // Change field functions
    // private
    // states: move_to or attack
    redraw_field(data) {
        console.log('redraw field');
        // bad switch, need to switch by data.type
        switch(data.type) {
            case 'move':
                this.move_unit(data.coords[0].y, data.coords[0].x, data.coords[1].y, data.coords[1].x);
            break;

            case 'attack':
                console.log('attack!!! charge!!!');
                // animate attack
                // change units params
            break;
        }
    }


    // Actions with units
    // private
    create_unit(texture, img_size, params, y, x) {
        let unit = new Unit(texture, img_size, params);
        this.grid.hexes[y][x].set_unit(unit);
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
}
