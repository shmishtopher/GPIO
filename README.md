# GPIO
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

```JavaScript
// Cylon Eyes

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
```
