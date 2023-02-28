/*
 * Copyright 2021, Google LLC
 *
 * SPDX-License-Identifier: Apache-2.0
 */
#![no_std]
#![no_main]

extern crate alloc;
extern crate libcantrip;
use libcantrip::sdk_init;

fn write(buf: &str) {
    for &b in buf.as_bytes() {
        unsafe {
            sel4_sys::seL4_DebugPutChar(b);
        }
    }
}

#[no_mangle]
pub fn main() {
    // NB: needed for format!.
    static mut HEAP: [u8; 4096] = [0; 4096];
    sdk_init(unsafe { &mut HEAP });
    write("Hello from rust app\n");
}
