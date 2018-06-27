const { GPIOChip, OUTPUT, HIGH } = require('../dist/ar_gpio.js')

const chip0 = new GPIOChip(0)
const line17 = chip0.request(17, OUTPUT)

line17.set(HIGH)
console.log(line17.get())
console.log(line17.info)

while (true) {}
