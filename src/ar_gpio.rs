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

use libc::c_char;
use libc::uint32_t;
use std::fs::File;
use std::os::unix;

#[repr(C)]
struct gpiochip_info {
  name: [c_char; 32],
  label: [c_char; 32],
  lines: uint32_t
}

#[repr(C)]
struct gpioline_info {
  line_offset: uint32_t,
  flags: uint32_t,
  name: [c_char; 32],
  consumer: [c_char; 32]
}

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

ioctl!(read get_chipinfo with 0xB4, 0x01; gpiochip_info);
ioctl!(readwrite get_lineinfo with 0xB4, 0x01; gpioline_info);

struct GPIOChip {
  file: File,
  name: String,
  label: String,
  lines: u32
}

struct GPIOHandle {
  file: File,
  gpio: u32,
  consumer: String,
  flags: uint32_t
}

impl GPIOChip {
  fn new (path: &std::path::Path) -> std::io::Result<GPIOChip> {
    
  }

  fn info (fd: unix::io::RawFd) -> std::io::Result(String, String, u32) {

  }

  fn request () {

  }
}