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
pub struct gpiochip_info {
  pub name: [i8; 32],
  pub label: [i8; 32],
  pub lines: u32
}

#[repr(C)]
pub struct gpioline_info {
  pub line_offset: u32,
  pub flags: u32,
  pub name: [i8; 32],
  pub consumer: [i8; 32]
}

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

ioctl_read!(get_chipinfo, 0xB4, 0x01, gpiochip_info);
ioctl_readwrite!(get_lineinfo, 0xB4, 0x02, gpioline_info);
ioctl_readwrite!(get_linehandle, 0xB4, 0x03, gpiohandle_request);
ioctl_readwrite!(get_line_values, 0xB4, 0x08, gpiohandle_data);
ioctl_readwrite!(set_line_values, 0xB4, 0x09, gpiohandle_data);

#[no_mangle]
pub extern fn gpiochip_info (fd: i32) -> *mut gpiochip_info {
  let mut info = gpiochip_info {
    name: [0; 32],
    label: [0; 32],
    lines: 0
  };

  get_chipinfo(fd, &mut info);
  unsafe { std::mem::transmute(Box::new(info)) }
}

#[no_mangle]
pub extern fn gpiochip_info_name (ptr: *mut gpiochip_info) -> [i8; 32] {
  let _gpiochip_info = unsafe { *ptr };
  _gpiochip_info.name
}

#[no_mangle]
pub extern fn gpiochip_info_label (ptr: *mut gpiochip_info) -> [i8; 32] {
  let _gpiochip_info = unsafe { *ptr };
  _gpiochip_info.label
}

#[no_mangle]
pub extern fn gpiochip_info_lines (ptr: *mut gpiochip_info) -> u32 {
  let _gpiochip_info = unsafe { *ptr };
  _gpiochip_info.lines
}



#[no_mangle]
pub extern fn gpioline_info_line_offset (ptr: *mut gpioline_info) -> u32 {
  let _gpioline_info = unsafe { *ptr };
  _gpioline_info.line_offset
}

#[no_mangle]
pub extern fn gpioline_info_flags (ptr: *mut gpioline_info) -> u32 {
  let _gpioline_info = unsafe { *ptr };
  _gpiochip_info.flags
}

#[no_mangle]
pub extern fn gpioline_info_name (ptr: *mut gpioline_info) -> [i8; 32] {
  let _gpioline_info = unsafe { *ptr };
  _gpioline_info.name
}

#[no_mangle]
pub extern fn gpioline_info_consumer (ptr: *mut gpioline_info) -> [i8; 32] {
  let _gpioline_info = unsafe { *ptr };
  _gpioline_info.consumer
}



#[no_mangle]
pub extern fn gpiohandle_lineoffsets (ptr: *mut gpiohandle_data) -> [u32; 64] {
  let _gpiohandle = unsafe { *ptr };
  _gpiohandle.lineoffsets
}

#[no_mangle]
pub extern fn gpiohandle_flags (ptr: *mut gpiohandle_data) -> u32 {
  let _gpiohandle = unsafe { *ptr };
  _gpiohandle.flags
}

#[no_mangle]
pub extern fn gpiohandle_fd (ptr: *mut gpiohandle_data) -> i32 {
  let _gpiohandle = unsafe { *ptr };
  _gpiohandle.fd
}