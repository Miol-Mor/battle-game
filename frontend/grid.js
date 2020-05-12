class Hex extends PIXI.Graphics {
    constructor(side, border_width, border_color) {
        super();
        this.lineStyle(border_width, border_color);

        // array of points of hex to draw
        this.points = this.hex_points(side);
        //Use drawPolygon to define the hex as a path array of x/y positions
        this.drawPolygon(this.points);
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
}



class Hex_grid {
    BORDER_COLOR = 0x000000;
    BORDER_WIDTH = 1;
    FILL_COLOR = 0x1020b0;

    // stage - where to draw grid (PIXI.Container)
    // row_n, col_n - number of rows and columns in grid
    // cellsize - side of a hexcell (length of its edge)
    // x_offset, y_offset - left and top offsets inside stage
    constructor(stage, row_n = 6, col_n = 8, cell_size = 50, x_offset = 50, y_offset = 50) {
        this.stage = stage;
        this.row_n = row_n;
        this.col_n = col_n;
        this.cell_size = cell_size;
        this.x_offset = x_offset;
        this.y_offset = y_offset;
        // cells - matrix of cells (call - cells[y][x], where y - number of row, x - number of column)
        this.cells = [];
    }


    // draw grid on the stage
    draw() {
        let grid_container = new PIXI.Container();
        this.stage.addChild(grid_container);
        grid_container.position.set(this.x_offset, this.y_offset);

        let side = this.cell_size;
        for (let y = 0; y < this.row_n; y++) {
            this.cells[y] = [];
            for (let x = 0; x < this.col_n; x++) {
                let cur_hex = new Hex(this.cell_size, this.BORDER_WIDTH, this.BORDER_COLOR);
                
                let y_offset = side;
                let x_offset = side * Math.sqrt(3) / 2;
                if (y % 2 == 0) {
                    x_offset = side * Math.sqrt(3);
                }
                let x_coord = (side * Math.sqrt(3)) * x + x_offset;
                let y_coord = (side * 3 / 2) * y + y_offset;
                
                cur_hex.position.set(x_coord, y_coord);
                grid_container.addChild(cur_hex);
                this.cells[y].push(cur_hex);
            }
        }
    }


    // fill hex in row y, column x with FILLCOLOR
    fill_hex(y, x) {
        let cur_hex = this.cells[y][x];

        cur_hex.clear();
        cur_hex.lineStyle(1, this.BORDER_COLOR, 1);
        cur_hex.beginFill(this.FILL_COLOR);
        cur_hex.drawPolygon(cur_hex.points);
        cur_hex.endFill();
    }

    
}