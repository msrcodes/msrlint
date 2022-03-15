'use strict'

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
    if (person.name == "Santa Claus") {
        console.log("Ho Ho Ho!")
    } else {
        console.log(`Hello ${person.name}`)
    }
}

sayHi(people[0])