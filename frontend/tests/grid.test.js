import * as PIXI from 'pixi.js';
import { Hex_grid, Hex } from '../src/grid.js';

test('Default grid', () => {
    let app = new PIXI.Application();
    document.body.appendChild(app.view);
    let grid = new Hex_grid(app);
    expect(grid.num_x).toBe(8);
});
