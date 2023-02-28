/*
 * Copyright 2022, Google LLC
 *
 * SPDX-License-Identifier: Apache-2.0
 */
#![no_std]
#![no_main]

// Demo hookup of cantrip-os-logger to sdk_log (eventually move to libcantrip).

extern crate alloc;
extern crate libcantrip;
use alloc::format;
use cantrip_os_common::allocator;
use sdk_interface::*;
use libcantrip::sdk_init;
use log::info;


#[no_mangle]
pub fn main() {
    // NB: need the allocator for error formatting.
    static mut HEAP: [u8; 4096] = [0; 4096];
    unsafe {
        allocator::ALLOCATOR.init(HEAP.as_mut_ptr() as _, HEAP.len());
    }
    info!("bye bye");
    let _ = match sdk_ping() {
        Ok(_) => sdk_log("ping!"),
        Err(e) => sdk_log(&format!("sdk_ping failed: {:?}", e)),
    };
    let _ = sdk_log("I am a Rust app, hear me log!");
}
