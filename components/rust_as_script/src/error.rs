/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Functions to throw JavaScript exceptions from Rust.

#![deny(missing_docs)]

use jsapi::{JSContext};
use libc;
use std::ffi::CString;
use std::{mem, os, ptr};

/// Throw a `TypeError` with the given message.
pub unsafe fn throw_type_error(cx: *mut JSContext, error: &str) {
        unimplemented!();
}

/// Throw a `RangeError` with the given message.
pub unsafe fn throw_range_error(cx: *mut JSContext, error: &str) {
        unimplemented!();
}
