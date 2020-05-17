class Unit {
    constructor(texture, grid, y, x, params) {
        this.sprite = new PIXI.Sprite(texture);
        this.grid = grid;
        this.y = y;
        this.x = x;
        this.params = params;

        this.hex = this.grid.hexs[this.y][this.x];
        
        this.sprite.anchor.set(0.5, 0.5);
        this.scale_sprite(this.sprite, this.grid.hex_size);
        this.set_n_draw();
    }

    // scale sprite to fit size
    scale_sprite(sprite, size) {
        let scale = size / Math.max(sprite.width, sprite.height);
        sprite.width *= scale;
        sprite.height *= scale;
    }

    // set unit in the hex and draw it
    set_n_draw() {
        this.hex.set_unit(this);
    }

    move_to(y, x) {
        this.hex.unset_unit();
        this.set_new_coords(y, x);
        this.hex.set_unit(this);
    }

    set_new_coords(y, x) {
        this.y = y;
        this.x = x;
        this.hex = this.grid.hexs[y][x];
    }
}