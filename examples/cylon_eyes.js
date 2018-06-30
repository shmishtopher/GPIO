/**
 * Copyright (C) 2018, Shmish <shmish90@gmail.com>
 *
 * This code is licensed under the MIT licence
 * found in the LICENCE file in the root directory
 * of this source tree.
 * 
 * This code demonstrates the classic "Cylon Eye"
 * effect.
 */


const { GPIOChip, OUTPUT, HIGH, LOW } = require('ar_gpio')

const chip = new GPIOChip(0)
const lines = [
  chip.line(14, OUTPUT),
  chip.line(15, OUTPUT),
  chip.line(18, OUTPUT),
  chip.line(23, OUTPUT),
  chip.line(24, OUTPUT)
]

function sleep (ms) {
  return new Promise(resolve => setTimeout(resolve, ms))
}

async function main () {
  while (true) {
    for (let i = 0; i < 4; i++) {
      await sleep(50)
      lines[i + 0].set(LOW) 
      lines[i + 1].set(HIGH)
    }

    for (let i = 4; i > 0; i--) {
      await sleep(50)
      lines[i + 0].set(LOW)
      lines[i - 1].set(HIGH)
    }
  }
}

main()