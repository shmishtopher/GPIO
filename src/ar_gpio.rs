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

use std::os::unix::io::RawFd;
use std::os::unix::io::FromRawFd;
use std::os::unix::io::AsRawFd;

mod request_flags {
  const INPUT: u32 =       0x01 << 0;
  const OUTPUT: u32 =      0x01 << 1;
  const ACTIVE_LOW: u32 =  0x01 << 2;
  const OPEN_DRAIN: u32 =  0x01 << 3;
  const OPEN_SOURCE: u32 = 0x01 << 4;
}

mod line_flags {
  const KERNEL: u32 =      0x01 << 0;
  const IS_OUT: u32 =      0x01 << 1;
  const ACTIVE_LOW: u32 =  0x01 << 2;
  const OPEN_DRAIN: u32 =  0x01 << 3;
  const OPEN_SOURCE: u32 = 0x01 << 4;
}

#[repr(C)]
pub struct gpiochip_info {
  name: [libc::c_char; 32],
  label: [libc::c_char; 32],
  lines: libc::uint32_t,
}

#[repr(C)]
pub struct gpioline_info {
  line_offset: libc::uint32_t,
  flags: libc::uint32_t,
  name: [libc::c_char; 32],
  consumer: [libc::c_char; 32],
}

#[repr(C)]
pub struct gpiohandle_request {
  lineoffsets: [libc::uint32_t; 64],
  flags: libc::uint32_t,
  default_values: [libc::uint8_t; 64],
  consumer_label: [libc::c_char; 32],
  lines: libc::uint32_t,
  fd: libc::c_int,
}

#[repr(C)]
pub struct gpiohandle_data {
  values: [libc::uint8_t; 64],
}

ioctl_read!(get_chipinfo, 0xB4, 0x01, gpiochip_info);
ioctl_readwrite!(get_lineinfo, 0xB4, 0x02, gpioline_info);
ioctl_readwrite!(get_linehandle, 0xB4, 0x03, gpiohandle_request);
ioctl_readwrite!(get_line_values, 0xB4, 0x08, gpiohandle_data);
ioctl_readwrite!(set_line_values, 0xB4, 0x09, gpiohandle_data);

pub struct GPIOChip {
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
  fn new (path: &std::path::Path) -> std::io::Result<GPIOChip> {
    let file = try!(std::fs::File::open(path));
    let (name, label, lines) = try!(GPIOChip::info(file.as_raw_fd()));
    Ok(GPIOChip {file: file, name: name, label: label, lines: lines})
  }

  fn info (fd: RawFd) -> std::io::Result<(String, String, u32)> {
    let mut data = gpiochip_info { name: [0; 32], label: [0; 32], lines: 0 };
    unsafe { get_chipinfo(fd, &mut data); }

    let name = unsafe {std::ffi::CStr::from_ptr(data.name.as_ptr())}
      .to_string_lossy()
      .into_owned();
    
    let label = unsafe {std::ffi::CStr::from_ptr(data.label.as_ptr())}
      .to_string_lossy()
      .into_owned();

    Ok((name, label, data.lines))
  }

  fn request (&self, consumer: &str, flags: u32, gpio: u32, default: u8) -> std::io::Result<GPIOHandle> {
    let mut request = gpiohandle_request {
      lineoffsets: [0; 64],
      flags: 0,
      default_values: [0; 64],
      consumer_label: [0; 32],
      lines: 0,
      fd: 0
    };

    request.lineoffsets[0] = gpio;
    request.flags = flags;
    request.default_values[0] = default;
    request.lines = 1;

    for i in 0..request.consumer_label.len() {
      if i >= consumer.len() { break; }
      request.consumer_label[i] = consumer.as_bytes()[i] as std::os::raw::c_char;
    }

    unsafe { get_linehandle(self.file.as_raw_fd(), &mut request); }
    
    Ok(GPIOHandle {
      file: unsafe {std::fs::File::from_raw_fd(request.fd)},
      consumer: consumer.to_string(),
      flags: flags,
      gpio: gpio
    })
  }
}

impl GPIOHandle {
  fn get (&self) -> std::io::Result<u8> {
    let mut data = gpiohandle_data { values: [0; 64] };
    unsafe { get_line_values(self.file.as_raw_fd(), &mut data); }
    Ok(data.values[0])
  }

  fn set (&self, state: u8) -> std::io::Result<()> {
    let mut data = gpiohandle_data { values: [0; 64] };
    data.values[0] = state;
    unsafe { set_line_values(self.file.as_raw_fd(), &mut data).unwrap(); }
    Ok(())
  }
}

#[no_mangle]
pub extern fn construct_gpiochip (p: *const libc::c_char) -> *mut GPIOChip {
  let c_str = unsafe { std::ffi::CStr::from_ptr(p) };
  let r_str = c_str.to_str().unwrap();
  let path = std::path::Path::new(r_str);

  unsafe { std::mem::transmute(Box::new(GPIOChip::new(&path))) }
}

// #[no_mangle]
// pub extern fn destruct_gpiochip (*mut GPIOChip) {}

// #[no_mangle]
// pub extern fn construct_gpioline () -> *mut GPIOHandle {}

// #[no_mangle]
// pub extern fn destruct_gpioline (*mut GPIOHandle) {}

// #[no_mangle]
// pub extern fn gpioline_get (*mut GPIOHandle) -> bool {}

// #[no_mangle]
// pub extern fn gpioline_set (*mut GPIOHandle, bool) {}