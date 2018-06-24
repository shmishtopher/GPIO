/**
 * Copyright (C) 2018, Shmish <shmish90@gmail.com>
 *
 * This code is licensed under the MIT licence
 * found in the LICENCE file in the root directory
 * of this source tree.
 */

#[macro_use]
extern crate nix;
extern crate libc;

mod RequestFlags {
  const INPUT: u32 =       0x01 << 0;
  const OUTPUT: u32 =      0x01 << 1;
  const ACTIVE_LOW: u32 =  0x01 << 2;
  const OPEN_DRAIN: u32 =  0x01 << 3;
  const OPEN_SOURCE: u32 = 0x01 << 4;
}

mod LineFlags {
  const KERNEL: u32 =      0x01 << 0;
  const IS_OUT: u32 =      0x01 << 1;
  const ACTIVE_LOW: u32 =  0x01 << 2;
  const OPEN_DRAIN: u32 =  0x01 << 3;
  const OPEN_SOURCE: u32 = 0x01 << 4;
}

#[repr(C)]
struct gpiochip_info {
  name: [libc::c_char; 32],
  label: [libc::c_char; 32],
  lines: libc::uint32_t,
}

#[repr(C)]
struct gpioline_info {
  line_offset: libc::uint32_t,
  flags: libc::uint32_t,
  name: [libc::c_char; 32],
  consumer: [libc::c_char; 32],
}

#[repr(C)]
struct gpiohandle_request {
  lineoffsets: [libc::uint32_t; 64],
  flags: libc::uint32_t,
  default_values: [libc::uint8_t; 64],
  consumer_label: [libc::c_char; 32],
  lines: libc::uint32_t,
  fd: libc::c_int,
}

#[repr(C)]
struct gpiohandle_data {
  values: [libc::uint8_t; 64],
}

ioctl!(read get_chipinfo with 0xB4, 0x01; gpiochip_info);
ioctl!(readwrite get_lineinfo with 0xB4, 0x02; gpioline_info);
ioctl!(readwrite get_linehandle with 0xB4, 0x03; gpiohandle_request);
ioctl!(readwrite get_line_values with 0xB4, 0x08; gpiohandle_data);
ioctl!(readwrite set_line_values with 0xB4, 0x09; gpiohandle_data);

struct GPIOChip {
  file: std::fs::File,
  name: String,
  label: String,
  lines: u32
}

struct GPIOHandle {
  file: std::fs::File,
  gpio: u32,
  consumer: String,
  flags: u32
}

impl GPIOChip {

}

impl GPIOHandle {
  fn get (&self) -> std::io::Result<u8> {
    let mut data = gpiohandle_data { values: [0; 64] };
    get_line_values(self.file.as_raw_fd(), &mut data);
    Ok(data.values[0])
  }

  fn set (&self, state: u8) -> std::io::Result<()> {
    let mut data = gpiohandle_data { values: [0; 64] };
    data.values[0] = value;
    set_line_values(self.file.as_raw_fd(), &mut data);
    Ok(())
  }
}

#[no_magle]
pub extern fn construct_gpiochip () -> *mut GPIOChip {}

#[no_mangle]
pub extern fn destruct_gpiochip () {}