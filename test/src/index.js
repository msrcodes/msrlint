'use strict';

import people from './people.json'

const foo = `this isn't allowed`

/**
 * A documentation comment
 * @param {number} x 
 * @param {number} y 
 * @returns x plus y
 */
function add(x, y) {
    return x + y
}

// A comment
function sayHi(person) {
    console.log(`Hello ${person.name}`)
}

sayHi(people[0])