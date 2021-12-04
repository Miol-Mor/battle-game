import * as PIXI from 'pixi.js';

class Hex extends PIXI.Graphics {
    constructor(x, y, side, border_width, border_color, default_color, mouseover_color, path_color, wall_color) {
        super();
        this.coords = { x: x, y: y };
        this.BORDER_WIDTH = border_width;
        this.BORDER_COLOR = border_color;
        this.DEFAULT_COLOR = default_color;
        this.MOUSEOVER_COLOR = mouseover_color;
        this.PATH_COLOR = path_color;
        this.WALL_COLOR = wall_color;

        this.state_selected = false;
        this.state_in_path = false;

        // array of points of hex to draw
        this.points = this.hex_points(side);
        //Use drawPolygon to define the hex as a path array of x/y positions
        this.polygon = new PIXI.Polygon(this.points);
        this.lineStyle(this.BORDER_WIDTH, this.BORDER_COLOR);
        this.drawPolygon(this.polygon);

        this.unit = null;
        this.content = null;
    }

    // return array of points of hex with center in (0, 0) point and given side
    hex_points(side) {
        let points = [
            - side * Math.sqrt(3) / 2, side / 2,
            0, side,
            side * Math.sqrt(3) / 2, side / 2,
            side * Math.sqrt(3) / 2, - side / 2,
            0, - side,
            - side * Math.sqrt(3) / 2, - side / 2
        ];
        return points;
    }

    fill() {
        let color = this.DEFAULT_COLOR;

        if (this.state_selected) {
            color = this.MOUSEOVER_COLOR;
        } else if (this.state_in_path) {
            color = this.PATH_COLOR;
        } else if (this.content) {
            color = this.WALL_COLOR;
        }

        this.clear();
        this.lineStyle(this.BORDER_WIDTH, this.BORDER_COLOR, 1);
        this.beginFill(color);
        this.drawPolygon(this.points);
        this.endFill();
    }

    set_content(content) {
        this.content = content;
        this.fill();
    }

    // set and draw unit here
    set_unit(unit) {
        this.unit = unit;
        this.draw_unit(unit);
    }

    // private
    // draw unit here
    draw_unit(unit) {
        this.addChild(unit.sprite);
    }

    unset_unit() {
        this.unit = null;
    }

    erase_unit() {
        this.removeChild(this.unit.sprite);
    }

    change_unit(params) {
        this.unit.params = params;
    }
}


export class Hex_grid {
    // stage - where to draw grid (PIXI.Container)
    // row_n, col_n - number of rows and columns in grid
    // cellsize - side of a hexcell (length of its edge)
    // x_offset, y_offset - left and top offsets inside stage
    constructor(stage, num_x = 8, num_y = 6, x_offset = 150, y_offset = 50) {
        this.stage = stage;
        this.num_x = num_x;
        this.num_y = num_y;
        // Magic formula to make fields size fit the screen
        this.hex_size = Number(Math.min((window.innerWidth - x_offset) / num_x, (window.innerHeight - y_offset) / num_y * 2 / Math.sqrt(3)) / 2);
        this.x_offset = x_offset;
        this.y_offset = y_offset;
        // hexes - matrix of hexes (call - hexes[y][x], where y - number of row, x - number of column)
        this.hexes = [];
        // graphic constants
        this.BORDER_WIDTH = 1;
        // Colors
        this.BORDER_COLOR = 0x000000; // black
        this.DEFAULT_COLOR = 0xd6b609; // yellow
        this.MOUSEOVER_COLOR = 0x04A348; // green
        this.PATH_COLOR = 0x7b8485; // lite gray
        this.WALL_COLOR = 0x073d44; // dark
    }

    // draw grid on the stage
    draw() {
        let grid_container = new PIXI.Container();
        this.stage.addChild(grid_container);
        grid_container.position.set(this.x_offset, this.y_offset);

        let side = this.hex_size;
        for (let x = 0; x < this.num_x; x++) {
            this.hexes[x] = [];
            for (let y = 0; y < this.num_y; y++) {
                let cur_hex = new Hex(x, y, this.hex_size,
                    this.BORDER_WIDTH, this.BORDER_COLOR,
                    this.DEFAULT_COLOR, this.MOUSEOVER_COLOR, this.PATH_COLOR, this.WALL_COLOR,
                );

                let y_offset = side;
                let x_offset = side * Math.sqrt(3) / 2;
                if (y % 2 === 0) {
                    x_offset = side * Math.sqrt(3);
                }
                let x_coord = (side * Math.sqrt(3)) * x + x_offset;
                let y_coord = (side * 3 / 2) * y + y_offset;

                cur_hex.position.set(x_coord, y_coord);
                grid_container.addChild(cur_hex);
                this.hexes[x].push(cur_hex);
            }
        }
    }

    // dev
    // private
    draw_hex(x, y) {
        let cur_hex = this.hexes[x][y];

        cur_hex.clear();
        cur_hex.lineStyle(this.BORDER_WIDTH, this.BORDER_COLOR, 1);
        cur_hex.beginFill(this.DEFAULT_COLOR);
        cur_hex.drawPolygon(cur_hex.points);
        cur_hex.endFill();
    }

    reset_in_path() {
        this.hexes.forEach(hexes => {
            hexes.forEach(hex => {
                if (hex.state_in_path) {
                    hex.state_in_path = false;
                    hex.fill();
                }
            });
        });
    }
}
