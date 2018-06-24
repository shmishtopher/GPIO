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

#[repr(C)]
struct gpiochip_info {
  name: [libc::c_char; 32],
  label: [libc::c_char; 32],
  lines: libc::uint32_t
}

#[repr(C)]
struct gpioline_info {
  line_offset: libc::uint32_t,
  flags: libc::uint32_t,
  name: [libc::c_char; 32],
  consumer: [libc::c_char; 32]
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

mod RequestFlags {
  const INPUT: uint32_t =       0x01 << 0;
  const OUTPUT: uint32_t =      0x01 << 1;
  const ACTIVE_LOW: uint32_t =  0x01 << 2;
  const OPEN_DRAIN: uint32_t =  0x01 << 3;
  const OPEN_SOURCE: uint32_t = 0x01 << 4;
}

mod LineFlags {
  const KERNEL: uint32_t =      0x01 << 0;
  const IS_OUT: uint32_t =      0x01 << 1;
  const ACTIVE_LOW: uint32_t =  0x01 << 2;
  const OPEN_DRAIN: uint32_t =  0x01 << 3;
  const OPEN_SOURCE: uint32_t = 0x01 << 4;
}

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
  flags: uint32_t
}

impl GPIOChip {
  fn new (path: &std::path::Path) -> std::io::Result<GPIOChip> {
    
  }

  fn info (fd: unix::io::RawFd) -> std::io::Result(String, String, u32) {
    let mut info = gpiochip_info {
      name: [0; 32],
      label: [0; 32],
      lines: 0
    };

    get_chipinfo(fd, &mut info);

    let name = 
  }

  fn request () {

  }
}