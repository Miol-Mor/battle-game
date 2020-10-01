import * as PIXI from 'pixi.js';

export class Unit {
    constructor(texture, img_size, params) {
        this.sprite = new PIXI.Sprite(texture);
        this.params = params;

        this.sprite.anchor.set(0.5, 0.5);
        this.scale_sprite(img_size);

        this.origin_img_size = img_size;
        this.pulse_state = false;
        this.pulse_count = 1;
    }

    // scale sprite to fit size
    scale_sprite(size) {
        let scale = size / Math.max(this.sprite.width, this.sprite.height);
        this.sprite.width *= scale;
        this.sprite.height *= scale;
    }

    start_pulse() {
        this.pulse_state = true;
    }

    stop_pulse() {
        this.pulse_state = false;
        this.pulse_count = 1;
        this.scale_sprite(this.origin_img_size);
    }

    pulse() {
        if (this.pulse_state) {
            this.pulse_count += 0.05;

            this.sprite.scale.x = Math.sin(this.pulse_count);
            this.sprite.scale.y = Math.sin(this.pulse_count);
        }
    }
}
