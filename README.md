# AR_GPIO
:gear: GPIO Pin control in JS using the gpiochip character device

[WIP]


```JavaScript
const { GPIOChip, HIGH, LOW, OUTPUT } = require('ar_gpio')

const chip = new GPIOChip(0)
const line1 = chip.request(15, OUTPUT)
const line2 = chip.request(17, OUTPUT)

line1.set(HIGH)
line2.set(LOW)
```
