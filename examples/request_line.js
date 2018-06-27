const { GPIOChip } = require('../dist/ar_gpio.js')

const chip0 = new GPIOChip(0)
const line7 = chip0.request(32, 1 << 1)

console.log(chip0, line7)
