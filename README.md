# AR_GPIO
:gear: GPIO Pin control in JS using the gpiochip character device

[WIP]


```JavaScript
const { GPIOChip, HIGH, LOW } = require('ar_gpio')

const chip = new GPIOChip(0)
const line1 = chip.request(1)
const line2 = chip.request(2)

line1.set(HIGH)
line2.set(LOW)
```
