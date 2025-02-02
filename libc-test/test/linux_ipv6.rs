#![allow(bad_style, improper_ctypes, unused, deprecated)]

extern crate libc;
use libc::*;

#[cfg(target_os = "linux")]
include!(concat!(env!("OUT_DIR"), "/linux_ipv6.rs"));

#[cfg(not(target_os = "linux"))]
#[cfg_attr(feature = "aggressive-inline", inline(always))]
fn main() {
    println!("PASSED 0 tests");
}
