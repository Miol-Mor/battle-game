class Hex_grid {
    BORDER_COLOR = 0x000000;
    BORDER_WIDTH = 1;
    FILL_COLOR = 0x1020b0;

    // stage - where to draw grid (PIXI.Container)
    // m, n - number of rows and columns in grid
    // cellsize - size of hexcell
    // x_offset, y_offset - left and top offsets inside stage
    // cells - matrix of cells (call - cells[y][x])
    constructor(stage, m = 6, n = 4, cell_size = 50, x_offset = 50, y_offset = 50) {
        this.stage = stage;
        this.m = m;
        this.n = n;
        this.cell_size = cell_size;
        this.x_offset = x_offset;
        this.y_offset = y_offset;
        this.cells = [];
    }
    

    // returns hex object with side = r
    create_hex(r) {
        let hex = new PIXI.Graphics();
        
        hex.lineStyle(this.BORDER_WIDTH, this.BORDER_COLOR);

        //Use `drawPolygon` to define the hex as a path array of x/y positions
        let points = [
            - r * Math.sqrt(3) / 2, r / 2,
            0, r,
            r * Math.sqrt(3) / 2, r / 2,
            r * Math.sqrt(3) / 2, - r / 2,
            0, - r,
            - r * Math.sqrt(3) / 2, - r / 2
        ];
        hex.drawPolygon(points);

        let hex_obj = {};
        hex_obj.hex = hex; // Graphics object
        hex_obj.points = points; // array of points
        return hex_obj;
    }


    // draws grid on the stage
    draw() {
        let grid_container = new PIXI.Container();
        this.stage.addChild(grid_container);
        grid_container.position.set(this.x_offset, this.y_offset);

        let r = this.cell_size;
        let n = this.n,
            m = this.m;
        let cells = this.cells;
        for (let y = 0; y < n; y++) {
            cells[y] = [];
            for (let x = 0; x < m; x++) {
                let hex_obj = this.create_hex(r);
                let hex = hex_obj.hex;
                
                let y_offset = r;
                let x_offset = r * Math.sqrt(3) / 2;
                if (y % 2 == 0) {
                    x_offset = r * Math.sqrt(3);
                }
                let x_coord = (r * Math.sqrt(3)) * x + x_offset;
                let y_coord = (r * 3 / 2) * y + y_offset;
                hex.position.set(x_coord, y_coord);
                grid_container.addChild(hex);
                cells[y].push(hex_obj);
            }
        }
    }


    // fill hex in row y, column x with FILLCOLOR
    fill_hex(y, x) {
        let hex_obj = this.cells[y][x];
        let hex = hex_obj.hex;
        let points = hex_obj.points;        
        
        hex.clear();
        hex.lineStyle(1, this.BORDER_COLOR, 1);
        hex.beginFill(this.FILL_COLOR);
        hex.drawPolygon(points);
        hex.endFill();
    }

    
}