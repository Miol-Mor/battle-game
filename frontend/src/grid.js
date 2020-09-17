import * as PIXI from 'pixi.js';

class Hex extends PIXI.Graphics {
    constructor(x, y, side, BORDER_WIDTH, BORDER_COLOR, FILL_COLOR) {
        super();
        this.coords = {x: x, y: y};
        this.BORDER_WIDTH = BORDER_WIDTH;
        this.BORDER_COLOR = BORDER_COLOR;
        this.FILL_COLOR = FILL_COLOR;

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
        this.clear();
        this.lineStyle(this.BORDER_WIDTH, this.BORDER_COLOR, 1);
        this.beginFill(this.FILL_COLOR);
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
    constructor(stage, num_x = 8, num_y = 6, hex_size = 50, x_offset = 50, y_offset = 50) {
        this.stage = stage;
        this.num_x = num_x;
        this.num_y = num_y;
        this.hex_size = hex_size;
        this.x_offset = x_offset;
        this.y_offset = y_offset;
        // hexes - matrix of hexes (call - hexes[y][x], where y - number of row, x - number of column)
        this.hexes = [];
        // graphic constants
        this.BORDER_COLOR = 0x000000;
        this.BORDER_WIDTH = 1;
        this.FILL_COLOR = 0x1020b0;
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
                let cur_hex = new Hex(x, y, this.hex_size, this.BORDER_WIDTH, this.BORDER_COLOR, this.FILL_COLOR);

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
        cur_hex.beginFill(0xd6b609);
        cur_hex.drawPolygon(cur_hex.points);
        cur_hex.endFill();
    }
}
