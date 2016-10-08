/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/. */

#![feature(filling_drop)]
#![feature(link_args)]
#![feature(unsafe_no_drop_flag)]
#![feature(const_fn)]

#![allow(drop_with_repr_extern)]
#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, improper_ctypes)]
#![allow(unused_variables)]

#[macro_use]
extern crate heapsize;
#[macro_use]
extern crate lazy_static;
extern crate libc;
#[macro_use]
extern crate log;
extern crate num_traits;


#[macro_use]
pub mod rust;
mod consts;
pub mod conversions;
pub mod error;
pub mod glue;
pub mod jsval;
pub mod jsapi;

pub use consts::*;

use jsapi::JSContext;
use jsval::JSVal;

#[inline(always)]
pub unsafe fn JS_CALLEE(_cx: *mut JSContext, vp: *mut JSVal) -> JSVal {
        unimplemented!();
}


