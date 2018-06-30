/**
 * Copyright (C) 2018, Shmish <shmish90@gmail.com>
 *
 * This code is licensed under the MIT licence
 * found in the LICENCE file in the root directory
 * of this source tree.
 */

const { Library, ref } = require('fastcall')
const { openSync } = require('fs')

const ffi = new Library(`${__dirname}/ar_gpio.so`)
  .function('pointer gpiochip_create (int32)')
  .function('uint8* gpiochip_name (pointer)')
  .function('uint8* gpiochip_label (pointer)')
  .function('uint32 gpiochip_lines (pointer)')
  .function('void gpiochip_destroy (pointer)')

  .function('pointer gpioline_create (int32, uint32)')
  .function('uint32 gpioline_offset (pointer)')
  .function('uint32 gpioline_flags (pointer)')
  .function('uint8* gpioline_name (pointer)')
  .function('uint8* gpioline_consumer (pointer)')
  .function('void gpioline_destroy (pointer)')

  .function('int32 gpiohandle_request (int32, uint32, uint32)')
  .function('void gpioline_set (int32, uint8)')
  .function('uint8 gpioline_get (int32)')


const HIGH = Symbol.for('@Shmish/GPIO_HIGH')
const LOW = Symbol.for('@Shmish/GPIO_LOW')


class GPIOChipInfo {
  constructor (file) {
    const $ptr = ffi.interface.gpiochip_create(file)

    this.name = ref.readCString(ffi.interface.gpiochip_name($ptr))
    this.label = ref.readCString(ffi.interface.gpiochip_label($ptr))
    this.lines = ffi.interface.gpiochip_lines($ptr)

    ffi.interface.gpiochip_destroy($ptr)
    Object.freeze(this)
  }
}


class GPIOLineInfo {
  constructor (chip, line) {
    const $ptr = ffi.interface.gpioline_create(chip, line)

    this.offset = ffi.interface.gpioline_offset($ptr)
    this.flags = ffi.interface.gpioline_flags($ptr)
    this.name = ref.readCString(ffi.interface.gpioline_name($ptr))
    this.consumer = ref.readCString(ffi.interface.gpioline_consumer($ptr))

    ffi.interface.gpioline_destroy($ptr)
    Object.freeze(this)
  }
}


class GPIOChip {
  constructor (int) {
    this.fd = openSync(`/dev/gpiochip${int}`, 0o666)
    this.info = new GPIOChipInfo(this.fd)
  }

  get name () { return this.info.name }
  get label () { return this.info.label }
  get lines () { return this.info.lines }

  line (line, flags) {
    return new GPIOLine(
      ffi.interface.gpiohandle_request(this.fd, line, flags),
      this.fd,
      line
    )
  }
}


class GPIOLine {
  constructor (line_fd, chip_fd, line) {
    this.fd = line_fd
    this.info = new GPIOLineInfo(chip_fd, line)
  }

  get offset () { return this.info.offset }
  get flags () { return this.info.flags }
  get name () { return this.info.name }
  get consumer () { return this.info.consumer }

  ['get'] () {
    const state = ffi.interface.gpioline_get(this.fd)
    if (state === 1) return HIGH
    if (state === 0) return LOW
  }

  ['set'] (state) {
    if (state === HIGH || state === 1) ffi.interface.gpioline_set(this.fd, 1)
    if (state === LOW || state === 0) ffi.interface.gpioline_set(this.fd, 0)
  }
}


module.exports.GPIOChipInfo = GPIOChipInfo
module.exports.GPIOLineInfo = GPIOLineInfo
module.exports.GPIOChip = GPIOChip
module.exports.HIGH = HIGH
module.exports.LOW = LOW

module.exports.INPUT =       (1 << 0)
module.exports.OUTPUT =      (1 << 1)
module.exports.ACTIVE_LOW =  (1 << 2)
module.exports.OPEN_DRAIN =  (1 << 3)
module.exports.OPEN_SOURCE = (1 << 4)
