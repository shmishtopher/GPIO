#[macro_use]
extern crate nix;
extern crate libc;

#[repr(C)]
struct gpiochip_info {
  name: [libc::c_char; 32],
  label: [libc::c_char; 32],
  lines: libc::uint32_t
}
