// Copyright (C) Microsoft Corporation. All rights reserved.

//! A empty Rust crate, whose only purpose is to serve as a "build system" for a
//! resource-only DLL containing an OpenHCL IGVM file.
//!
//! The real magic is all in build.rs. See that file for more info.

#![cfg_attr(not(any(test, feature = "ci")), no_std)]

// required by the Rust compiler, even though we don't include any code
#[cfg(not(any(test, feature = "ci")))]
#[panic_handler]
fn panic_handler(_panic: &core::panic::PanicInfo<'_>) -> ! {
    unreachable!()
}