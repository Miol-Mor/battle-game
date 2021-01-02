class TraverseArray extends Array {
    constructor(arr) {
        if (Array.isArray(arr)) {
            super(arr);

            this.current = -1;
            Object.assign(this, arr);
        }
    }

    next() {
        return {
            value: this[++this.current],
            done: this.current >= this.length,
        };
    }
}

// TODO: Sure, it can be done simpler
// Iterate through array returning window of given size
// For example window_iterator([1, 2, 3, 4], 2) will produce following values:
// [1, 2]; [2, 3]; [3, 4]
// If window_length more then length of array, will produce nothing
export function window_iterator(array, window_length) {
    const iterator = new TraverseArray(array);
    let saved = [];

    const w_iterator = {
        next: function () {
            let iter_next = { value: null, done: true };

            // First run:
            // * Filling saved with null (to shift it away later)
            // * Add all values from iter from 0 to window_length - 1
            // * Then goes as usual
            if (!saved.length) {
                saved.push(null);

                for (let i = 0; i < window_length - 1; i++) {
                    iter_next = iterator.next();

                    saved.push(iter_next.value);
                }
            }

            iter_next = iterator.next();
            saved.shift();
            saved.push(iter_next.value);

            return {
                value: saved,
                done: iter_next.done,
            };
        },
        [Symbol.iterator]: function () { return this; },
    };

    return w_iterator;
}

// sleep for given period of time
export const sleep = (delay) => new Promise((resolve) => setTimeout(resolve, delay));
