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

mod RequestFlags {
  const INPUT: libc::uint32_t =       0x01 << 0;
  const OUTPUT: libc::uint32_t =      0x01 << 1;
  const ACTIVE_LOW: libc::uint32_t =  0x01 << 2;
  const OPEN_DRAIN: libc::uint32_t =  0x01 << 3;
  const OPEN_SOURCE: libc::uint32_t = 0x01 << 4;
}

mod LineFlags {
  const KERNEL: libc::uint32_t =      0x01 << 0;
  const IS_OUT: libc::uint32_t =      0x01 << 1;
  const ACTIVE_LOW: libc::uint32_t =  0x01 << 2;
  const OPEN_DRAIN: libc::uint32_t =  0x01 << 3;
  const OPEN_SOURCE: libc::uint32_t = 0x01 << 4;
}