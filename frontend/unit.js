class Unit {
    constructor(texture, img_size, params) {
        this.sprite = new PIXI.Sprite(texture);
        this.params = params;

        this.sprite.anchor.set(0.5, 0.5);
        this.scale_sprite(this.sprite, img_size);
    }

    // scale sprite to fit size
    scale_sprite(sprite, size) {
        let scale = size / Math.max(sprite.width, sprite.height);
        sprite.width *= scale;
        sprite.height *= scale;
    }
}
