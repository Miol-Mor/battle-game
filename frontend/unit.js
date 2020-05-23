class Unit {
    constructor(texture, img_size, params) {
        this.sprite = new PIXI.Sprite(texture);
        this.params = params;

        this.sprite.anchor.set(0.5, 0.5);
        this.scale_sprite(img_size);
    }

    // scale sprite to fit size
    scale_sprite(size) {
        let scale = size / Math.max(this.sprite.width, this.sprite.height);
        this.sprite.width *= scale;
        this.sprite.height *= scale;
    }
}
