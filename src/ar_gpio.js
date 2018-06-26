const { Library } = require('fastcall')

const ffi = new Library(`${__dirname}/ar_gpio.so`)
  .function('pointer gpiochip_create (int32)')
  .function('string gpiochip_name (pointer)')
  .function('string gpiochip_label (pointer)')
  .function('uint32 gpiochip_lines (pointer)')
  .function('void gpiochip_destroy (pointer)')

  .function('pointer gpioline_create (int32)')
  .function('uint32 gpioline_offset (pointer)')
  .function('uint32 gpioline_flags (pointer)')
  .function('string gpioline_name (pointer)')
  .function('string gpioline_consumer (pointer)')
  .function('void gpioline_destroy (pointer)')


class GPIOChipInfo {
  constructor (file) {
    const $ptr = ffi.interface.gpiochip_create(file)

    this.name = ffi.interface.gpiochip_name($ptr)
    this.label = ffi.interface.gpiochip_label($ptr)
    this.lines = ffi.interface.gpiochip_lines($ptr)

    ffi.interface.gpiochip_destroy($ptr)
    Object.freeze(this)
  }
}


class GPIOLineInfo {
  constructor (file) {
    const $ptr = ffi.interface.gpioline_create(file)

    this.offset = ffi.interface.gpioline_offset($ptr)
    this.flags = ffi.interface.gpioline_flags($ptr)
    this.name = ffi.interface.gpioline_name($ptr)
    this.consumer = ffi.interface.gpioline_consumer($ptr)

    ffi.interface.gpioline_destroy($ptr)
    Object.freeze(this)
  }
}


module.exports.GPIOChipInfo = GPIOChipInfo
module.exports.GPIOLineInfo = GPIOLineInfo