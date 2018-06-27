const { GPIOChip } = require('../dist/ar_gpio.js')

const chip0 = new GPIOChip(0)
const line7 = chip0.request(7, 0x01)

console.log(chip0, line7)
