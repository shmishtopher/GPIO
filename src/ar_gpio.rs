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
pub struct gpiochip_info {
  pub name: [u8; 32],
  pub label: [u8; 32],
  pub lines: u32
}

#[repr(C)]
pub struct gpioline_info {
  pub line_offset: u32,
  pub flags: u32,
  pub name: [u8; 32],
  pub consumer: [u8; 32]
}

#[repr(C)]
pub struct gpiohandle_request {
  pub lineoffsets: [u32; 64],
  pub flags: u32,
  pub default_values: [u8; 64],
  pub consumer_label: [u8, 32],
  pub lines: u32,
  pub fd: i32
}

#[repr(C)]
pub struct gpiohandle_data {
  pub values: [u8; 64]
}

ioctl_read!(get_chip_info, 0xB4, 0x01, gpiochip_info);
ioctl_readwrite!(get_line_info, 0xB4, 0x02, gpioline_info);
ioctl_readwrite!(get_line_handle, 0xB4, 0x02, gpiohandle_request);
ioctl_readwrite!(get_line_values, 0xB4, 0x08, gpiohandle_data);
ioctl_readwrite!(set_line_values, 0xB4, 0x09, gpiohandle_data);


pub extern fn gpiochip_create (fd: i32) -> *mut gpiochip_info {
  let mut info = gpiochip_info {
    name: [0; 32],
    label: [0; 32],
    lines: 0
  };

  unsafe {
    get_chip_info(fd, &mut info);
    std::mem::transmute(Box::new(info))
  }
}

pub extern fn gpiochip_name (ptr: *mut gpiochip_info) -> [u8; 32] {
  let _gpiochip = unsafe { *ptr };
  _gpiochip.name
}

pub extern fn gpiochip_label (ptr: *mut gpiochip_info) -> [u8; 32] {
  let _gpiochip = unsafe { *ptr };
  _gpiochip.label
}

pub extern fn gpiochip_lines (ptr: *mut gpiochip_info) -> u32 {
  let _gpiochip = unsafe { *ptr };
  _gpiochip.lines
}

pub extern fn gpiochip_destroy (ptr: *mut gpiochip_info) {
  let _gpiochip: Box<gpiochip_info> = unsafe { std::mem::transmute(ptr) };
}


pub extern fn gpioline_create (fd: i32) -> *mut gpioline_info {
  let mut info = gpioline_info {
    line_offset: 0,
    flags: 0,
    name: [0; 32],
    consumer: [0; 32]
  }

  unsafe {
    get_line_info(fd, &mut info);
    std::meme::transmute(Box::new(info))
  }
}

pub extern fn gpioline_offset (ptr: *mut gpioline_info) -> u32 {
  let _gpioline = unsafe { *ptr };
  _gpioline.line_offset
}

pub extern fn gpioline_flags (ptr: *mut gpioline_info) -> u32 {
  let _gpioline = unsafe { *ptr };
  _gpioline.flags
}

pub extern fn gpioline_name (ptr: *mut gpioline_info) -> [u8; 32] {
  let _gpioline = unsafe { *ptr };
  _gpioline.name
}

pub extern fn gpioline_consumer (ptr: *mut gpioline_info) -> [u8; 32] {
  let _gpioline = unsafe { *ptr };
  _gpioline.consumer
}

pub extern fn gpioline_destroy (ptr: *mut gpioline_info) {
  let _gpioline: Box<gpioline_info> = unsafe { std::mem::transmute(ptr) };
}