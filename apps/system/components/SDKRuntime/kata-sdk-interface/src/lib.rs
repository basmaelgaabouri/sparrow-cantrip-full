// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! CantripOS SDK runtime interfaces

#![cfg_attr(not(test), no_std)]

pub mod error;

pub use error::SDKError;
pub use error::SDKRuntimeError;

/// Rust interface for the SDKRuntime.
///
/// This trait defines all of the same verbs we expect to support in the component
/// interface, for both client and server, since CAmkES does not (yet) know how
/// to generate Rust bindings.
///
/// On the server side, the impl of this trait is instantiated in the component
/// as a global mutable object where the incoming calls from the CAmkES C side
/// are wrapped.
///
/// On the client side, this trait is implemented using top-level functions,
/// wrapping their CAmkES C stubs.
pub trait SDKRuntimeInterface {
    /// Pings the SDK runtime, going from client to server and back via CAmkES IPC.
    fn ping(&self) -> Result<(), SDKError>;
}

/// Rust client-side wrapper for the autogenerated CAmkES ping method.
#[inline]
#[allow(dead_code)]
pub fn cantrip_sdk_ping() -> Result<(), SDKError> {
    extern "C" {
        fn sdk_runtime_sdk_ping() -> SDKRuntimeError;
    }

    unsafe { sdk_runtime_sdk_ping().into() }
}