const { GPIOChip } = require('../dist/ar_gpio.js')

const chip0 = new GPIOChip(0)
const line17 = chip0.request(17, 1 << 1)

line17.set(1)
console.log(line17.get())
console.log(line17.info)

while (true) {}
