/**
 * Copyright (C) 2018, Shmish <shmish90@gmail.com>
 *
 * This code is licensed under the MIT licence
 * found in the LICENCE file in the root directory
 * of this source tree.
 */

#[macro_use]
extern crate nix;

#[repr(C)]
pub struct gpiohandle_request {
  pub lineoffsets: [u32; 64],
  pub flags: u32,
  pub default_values: [u8; 64],
  pub consumer_label: [i8; 64],
  pub lines: u32,
  pub fd: i32
}

#[repr(C)]
pub struct gpiohandle_data {
  pub values: [u8; 64]
}

ioctl_readwrite!(get_linehandle, 0xB4, 0x03, gpiohandle_request);
ioctl_readwrite!(get_line_values, 0xB4, 0x08, gpiohandle_data);
ioctl_readwrite!(set_line_values, 0xB4, 0x09, gpiohandle_data);

#[no_mangle]
pub extern fn gpiohandle_request (fd: i32, line: i32, flags: u32) -> i32 {
  let mut request = gpiohandle_request {
    lineoffsets: [0; 64],
    flags: 0;
    default_values: [0; 64],
    consumer_label: [0; 64],
    lines: 0;
    fd: 0
  };

  request.lineoffsets[0] = line;
  request.flags = flags;

  unsafe {
    get_linehandle(fd, &mut request);
    request.fd
  }
}

#[no_mangle]
pub extern fn gpiohandle_set (fd: i32, value: bool) {
  let mut data = gpiohandle_data { values: [0; 64] };
  data.values[0] = if (value) { 1 } else { 0 };
  unsafe { set_line_values(fd, &mut data); }
}

#[no_mangle]
pub extern fn gpiohandle_get (fd: i32) -> bool {
  let mut data = gpiohandle_data { values: [0; 64] };
  unsafe { get_line_values(fd, &mut data); }
  if (data.values[0] == 0) { false } else { true }
}