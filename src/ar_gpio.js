const { Library } = require('fastcall')

const lib = new Library(`${__dirname}/ar_gpio.so`)
  .function('int32 gpiohandle_request (int32, int32, uint32)')
  .function('void gpiohandle_set (int32, bool)')
  .function('bool gpiohandle_get (int32)')
