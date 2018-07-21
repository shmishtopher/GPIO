# GPIO
:gear: GPIO Pin control in JS using the gpiochip character device

# Install
AR_GPIO is build on [`fastcall`](https://www.npmjs.com/package/fastcall), which uses cMake instead of node-gyp.  Install cMake before installing AR_GPIO
```
# Install C++ Toolchain
sudo apt-get update
sudo apt-get install build-essential
```
```
sudo apt-get install cmake  # AR_GPIO depends on cmake
```
```
npm i --save https://github.com/shmishtopher/GPIO  # Node dependency
```


# Example
![cylon eyes](examples/cyloneyes.png)
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
# Building
Building requires the Rust compiler.  The easist way to get RustC and Cargo is with `Rustup`
```
curl https://sh.rustup.rs -sSf | sh -s -- --help
```
With the Rust compiler installed you can build with the NPM script.
```
npm run build
```
