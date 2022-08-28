import * as PIXI from 'pixi.js';
import { Hex_grid } from './grid';
import { Unit } from './unit';
import { window_iterator, sleep } from './helpers';

export class Game {
    constructor() {
        // Websocket we use to contact with server
        this.socket = null;
        // app - PIXI aplication, used for this game
        this.app = null;

        this.STATES = {
            OUTSIDE: 'outside',
            SELECT: 'select',
            ACTION: 'action',
            ATTACK: 'attack',
            WAIT: 'wait',
            WATCH: 'watch',
        };

        // current game state
        this.state = this.STATES.OUTSIDE;
        // if game has been started on the server?
        this.game_started = false;
        // to output client their number in queue and total number of clients
        this.queue_status = null;
        // info outputted in the top of the screen
        this.info = null;
        // show info on click or do action
        this.info_state = false;
        // needed for block players action during moving animation
        this.players_action_enabled = false;


        // graphic constants
        this.BACKGROUND_COLOR = 0xd6b609;

        // TODO: make cmd_map map with auto-generated fields (process_CMD_NAME)
        // set map of commands
        this.cmd_map = {};

        this.cmd_map.queue = this.process_queue;
        // reset game (or create a new one if doesn't exist)
        this.cmd_map.field = this.create_new_field;

        this.cmd_map.selecting = this.process_selecting;

        this.cmd_map.deselecting = function (data) {
            let hex = this.grid.hexes[data.target.x][data.target.y];
            if (hex.unit) {
                hex.unit.stop_pulse();
            }

            this.grid.reset_in_path();
        };

        this.cmd_map.moving = this.process_moving;
        this.cmd_map.attacking = this.process_attacking;

        this.cmd_map.hurt = this.process_hurt;
        this.cmd_map.die = this.process_die;
        this.cmd_map.update = this.process_update;

        this.cmd_map.state = function (data) {
            this.change_state(data.state);
        };

        this.cmd_map.end = function (data) {
            this.clear_app();
            this.set_defaults();
            this.create_queue_status();
            this.create_start_button();
            switch (data.state) {
                case 'win':
                    alert('YOU WIN!');
                    break;
                case 'lose':
                    alert('YOU LOSE!');
                    break;
                case 'disconnected':
                    alert('Game was aborted because one of players was disconnected');
                    break;
                default:
                    alert('Game ends, but something strange happened');
            }
        };

        this.cmd_map.error = function (data) {
            console.error(JSON.stringify(data));
        };

        this.cmd_map['GFY! :D'] = () => { alert('GFY! :D'); };
    }

    // (needed, because constructor cannot be async)
    async start() {
        console.log('Is mobile:', PIXI.utils.isMobile.any);
        this.create_stage();
        await this.load_images();
        this.create_socket();
        this.create_queue_status();
        this.create_start_button();
    }

    // create websocket to receive and send messages
    // private
    create_socket() {
        this.socket = new WebSocket(`ws://${process.env.WS_ADDRESS}:8088/ws/`);

        this.socket.onopen = function (e) {
            console.log("[open] Connection established");
        };

        this.socket.onmessage = this.process_message.bind(this);

        this.socket.onclose = function (event) {
            if (event.wasClean) {
                console.log(`[close] Connection closed cleanly, code=${event.code} reason=${event.reason}`);
            } else {
                // e.g. server process killed or network down
                // event.code is usually 1006 in this case
                console.log('[close] Connection died');
            }
        };

        this.socket.onerror = function (error) {
            console.log(`[error] ${error.message}`);
        };
    }


    // States operations
    // private
    change_state(state) {
        this.state = state;
        switch (state) {
            case this.STATES.ACTION:
                this.players_action_enabled = true;
            break;
            case this.STATES.WAIT, this.STATES.WATCH:
                this.players_action_enabled = false;
            break;
        }
        // console.log('players_action_enabled', this.players_action_enabled);
    }


    // Game cycle
    // process messages from the server
    // private
    process_message(event) {
        let data = JSON.parse(event.data);
        console.log(data);
        // outside clients can't process almost all commands
        const allowed_cmds = ['state', 'queue'];
        if (allowed_cmds.includes(data.cmd) || this.state !== this.STATES.OUTSIDE) {
            this.cmd_map[data.cmd].call(this, data);
            // TODO: not necessarry to call it on each message
            this.show_tooltip();
        }
    }

    // process users clicks
    // private
    process_click(event) {
        console.log('clicked', event.target.coords);
        this.send_to_backend('click', event.target.coords);
    }

    // process skip turn action
    process_skip_turn() {
        console.log('skip turn');
        this.send_to_backend('skip_turn');
    }

    // // process info button click
    process_info_click() {
        this.info_state = !this.info_state;
        switch (this.info_state) {
            case true:
                this.info_button.texture = this.app.loader.resources["info button clicked"].texture;
                break;
            case false:
                this.info_button.texture = this.app.loader.resources["info button"].texture;
                break;
            default:
                throw new Error('Wrong info_state:', this.info_state);
        }
        // console.log('info_state:', this.info_state);
    }

    process_start(event) {
        console.log('start game');
        this.socket.send(
            JSON.stringify({
                "cmd": 'start_game',
            })
        );
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
        this.hide_queue_status();
        this.create_info();
        this.create_skip_button();
        if (PIXI.utils.isMobile.any) {
            this.create_info_button();
        }
        this.set_hex_click_handlers();
    }

    // set start values of game fields
    set_defaults() {
        // grid - grid for this game (Hex_grid)
        this.grid = null;

        this.cur_hex = null;
    }

    // remove everything from the PIXI stage
    clear_app() {
        this.app.stage.removeChildren();
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
        this.app.loader.add('green unit', 'images/green unit.png');
        this.app.loader.add('white unit', 'images/white unit.png');
        this.app.loader.add('black unit', 'images/black unit.png');
        this.app.loader.add('skip button', 'images/skip button icon.png');
        this.app.loader.add('info button', 'images/info button icon.png');
        this.app.loader.add('info button clicked', 'images/info button clicked icon.png');
        this.app.loader.add('start game button', 'images/start button icon.png');

        return new Promise(resolve => {
            this.app.loader.load(function () {
                resolve("images loaded");
            });
        });
    }

    // private
    create_grid(field_data) {
        let grid = new Hex_grid(this.app.stage, field_data.num_x, field_data.num_y, 150, 50);
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
                // TODO: refactor this
                switch (hexes[i].unit.player) {
                    case 0:
                        texture = this.app.loader.resources["blue unit"].texture;
                    break;
                    case 1:
                        texture = this.app.loader.resources["red unit"].texture;
                    break;
                    case 2:
                        texture = this.app.loader.resources["green unit"].texture;
                    break;
                    case 3:
                        texture = this.app.loader.resources["white unit"].texture;
                    break;
                    default:
                        texture = this.app.loader.resources["black unit"].texture;
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

    create_queue_status() {
        this.queue_status = new PIXI.Text();
        this.app.stage.addChild(this.queue_status);
    }

    hide_queue_status() {
        this.queue_status.visible = false;
    }

    create_start_button() {
        this.start_button = new PIXI.Sprite(this.app.loader.resources["start game button"].texture);
        this.start_button.buttonMode = true;
        this.start_button.anchor.set(0);
        this.start_button.position.x = 30;
        this.start_button.position.y = 100;
        this.start_button.interactive = true;
        this.start_button.visible = false;

        this.app.stage.addChild(this.start_button);
        this.start_button.on('pointerdown', this.process_start.bind(this));
    }

    hide_start_button() {
        this.start_button.visible = false;
    }

    show_start_button() {
        this.start_button.visible = true;
    }

    create_skip_button() {
        this.skip_button = new PIXI.Sprite(this.app.loader.resources["skip button"].texture);
        this.skip_button.buttonMode = true;
        this.skip_button.anchor.set(0);
        this.skip_button.position.x = 10;
        this.skip_button.position.y = 200;
        this.skip_button.width = 60;
        this.skip_button.height = 60;
        this.skip_button.interactive = true;

        this.app.stage.addChild(this.skip_button);
    }

    create_info_button() {
        this.info_button = new PIXI.Sprite(this.app.loader.resources["info button"].texture);
        this.info_button.buttonMode = true;
        this.info_button.anchor.set(0);
        this.info_button.position.x = 80;
        this.info_button.position.y = 200;
        this.info_button.width = 60;
        this.info_button.height = 60;
        this.info_button.interactive = true;

        this.app.stage.addChild(this.info_button);
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
            if (key === 'movements') continue;
            if (key === 'player') value += 1;
            let text_value = value;
            if (Array.isArray(value)) {
                text_value = value.join('-');
            }

            this.info.text += `${key}: ${text_value}\n`;
        }
    }

    // private
    turn_info() {
        switch (this.state) {
            case this.STATES.WAIT:
                return 'Wait for opponents turn\n';
            case this.STATES.SELECT:
                return 'Your turn: Select unit\n';
            case this.STATES.ACTION:
                return 'Your turn: Move unit or attack\n';
            case this.STATES.ATTACK:
                return 'Your turn: Attack\n';
            case this.STATES.WATCH:
                return `You are spectator. Just enjoy watching!\n${this.queue_status.text}\n`;
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
                hex.on('pointerdown', (event) => {
                    this.process_mouseover(event);
                    if (!this.info_state) {
                        this.process_click(event);
                    }
                });
                hex.on('mouseover', this.process_mouseover.bind(this));
                hex.on('mouseout', this.process_mouseout.bind(this));
            }
        }

        this.skip_button.on('pointerdown', this.process_skip_turn.bind(this));
        if (PIXI.utils.isMobile.any) {
            this.info_button.on('pointerdown', this.process_info_click.bind(this));
        }
    }

    // private
    process_mouseover(event) {
        let hex = event.currentTarget;
        this.cur_hex = hex;
        if (!PIXI.utils.isMobile.any) {
            hex.state_selected = true;
            hex.fill();
        }

        this.show_tooltip();
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
        let hex = event.currentTarget;
        hex.state_selected = false;
        hex.fill();

        this.clear_info();
        this.cur_hex = null;
    }

    send_to_backend(cmd, target) {
        if (this.players_action_enabled) {
            this.socket.send(
                JSON.stringify({
                    "cmd": cmd,
                    "target": target,
                })
            );
        }
        else {
            console.log('Be patient');
        }
    }

    // Change field functions
    // private
    process_selecting(data) {
        this.grid.hexes[data.target.x][data.target.y].unit.start_pulse();

        data.highlight_hexes.forEach(hex_data => {
            let hex = this.grid.hexes[hex_data.x][hex_data.y];

            hex.state_in_path = true;
            hex.fill();
        });

        console.log('highlight: ', data.highlight_hexes);
    }

    async process_moving(data) {
        this.players_action_enabled = false;
        for (let submove of window_iterator(data.coords, 2)) {
            this.move_unit(submove[0].x, submove[0].y, submove[1].x, submove[1].y);
            await sleep(300);
        }

        this.grid.reset_in_path();
        this.players_action_enabled = true;
    }

    process_attacking(data) {
        console.log('attack!!! charge!!!');
        // animate attack
    }

    process_hurt(data) {
        console.log('process_hurt');
        data.hexes.forEach(hex => {
            console.log('hex: ', hex);
            this.change_hex(hex);
        });
    }

    process_die(data) {
        console.log('process_die');
        data.hexes.forEach(hex => {
            console.log('hex: ', hex);
            this.kill_unit(hex);
        });
    }

    process_update(data) {
        console.log('process_update');
        data.hexes.forEach(hex => {
            console.log('hex: ', hex);
            this.change_hex(hex);
        });
    }

    process_queue(data) {
        console.log('process_queue');
        this.queue_status.text = `Players connected: ${data.players_number}; Your number ${data.your_number}\n`;
        if (data.game_started) {
            this.queue_status.text += 'The game has been already started. Please wait for the end of game\n';
            this.hide_start_button();
        }
        else if (data.players_number < 2) {
            this.queue_status.text += 'Wait for other players\n';
            this.hide_start_button();
        }
        else {
            this.show_start_button();
        }
        this.game_started = data.game_started;
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
    move_unit(from_x, from_y, to_x, to_y) {
        let from_hex = this.grid.hexes[from_x][from_y];
        let to_hex = this.grid.hexes[to_x][to_y];
        let unit = from_hex.unit;

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
                if (!hex.content && !hex.unit) {
                    return hex.coords;
                }
            }
        }
        return null;
    }
}
