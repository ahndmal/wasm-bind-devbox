// defined-in-js.js

export class Cat {
    constructor() {
        this._age = 6;
    }

    get age() {
        return this._age;
    }

    set age(n) {
        return this._age = n;
    }

    render() {
        return `Cat: ${this.age}`;
    }
}