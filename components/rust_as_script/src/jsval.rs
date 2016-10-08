/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/. */


use jsapi::{JSObject, JSString, TraceKind};
use jsapi::Value;
use jsapi::JSValueType;

use libc::c_void;
use std::default::Default;
use std::mem;
use heapsize::HeapSizeOf;

pub type JSVal = Value;

known_heap_size!(0, JSVal);

#[inline(always)]
pub fn NullValue() -> JSVal {
        unimplemented!();
}

#[inline(always)]
pub fn UndefinedValue() -> JSVal {
        unimplemented!();
}

#[inline(always)]
pub fn Int32Value(i: i32) -> JSVal {
        unimplemented!();
}

#[cfg(target_pointer_width = "64")]
#[inline(always)]
pub fn DoubleValue(f: f64) -> JSVal {
        unimplemented!();
}

#[cfg(target_pointer_width = "32")]
#[inline(always)]
pub fn DoubleValue(f: f64) -> JSVal {
        unimplemented!();
}

#[inline(always)]
pub fn UInt32Value(ui: u32) -> JSVal {
        unimplemented!();
}

#[cfg(target_pointer_width = "64")]
#[inline(always)]
pub fn StringValue(s: &JSString) -> JSVal {
        unimplemented!();
}

#[cfg(target_pointer_width = "32")]
#[inline(always)]
pub fn StringValue(s: &JSString) -> JSVal {
        unimplemented!();
}

#[inline(always)]
pub fn BooleanValue(b: bool) -> JSVal {
        unimplemented!();
}

#[cfg(target_pointer_width = "64")]
#[inline(always)]
pub fn ObjectValue(o: &JSObject) -> JSVal {
        unimplemented!();
}

#[cfg(target_pointer_width = "32")]
#[inline(always)]
pub fn ObjectValue(o: &JSObject) -> JSVal {
        unimplemented!();
}

#[inline(always)]
pub fn ObjectOrNullValue(o: *mut JSObject) -> JSVal {
        unimplemented!();
}

#[cfg(target_pointer_width = "64")]
#[inline(always)]
pub fn PrivateValue(o: *const c_void) -> JSVal {
        unimplemented!();
}

#[cfg(target_pointer_width = "32")]
#[inline(always)]
pub fn PrivateValue(o: *const c_void) -> JSVal {
        unimplemented!();
}

impl JSVal {
    #[inline(always)]
    fn asBits(&self) -> u64 {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "64")]
    pub fn is_undefined(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "32")]
    pub fn is_undefined(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "64")]
    pub fn is_null(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "32")]
    pub fn is_null(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    pub fn is_null_or_undefined(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "64")]
    pub fn is_boolean(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "32")]
    pub fn is_boolean(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "64")]
    pub fn is_int32(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "32")]
    pub fn is_int32(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "64")]
    pub fn is_double(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "32")]
    pub fn is_double(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "64")]
    pub fn is_number(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "32")]
    pub fn is_number(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "64")]
    pub fn is_primitive(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "32")]
    pub fn is_primitive(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "64")]
    pub fn is_string(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "32")]
    pub fn is_string(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "64")]
    pub fn is_object(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "32")]
    pub fn is_object(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "64")]
    pub fn is_symbol(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "32")]
    pub fn is_symbol(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "64")]
    pub fn to_boolean(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "32")]
    pub fn to_boolean(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    pub fn to_int32(&self) -> i32 {
        assert!(self.is_int32());
        unimplemented!();
    }

    #[inline(always)]
    pub fn to_double(&self) -> f64 {
        unimplemented!();
    }

    #[inline(always)]
    pub fn to_number(&self) -> f64 {
        unimplemented!();
    }

    #[inline(always)]
    pub fn to_object(&self) -> *mut JSObject {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "64")]
    pub fn to_string(&self) -> *mut JSString {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "32")]
    pub fn to_string(&self) -> *mut JSString {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "64")]
    pub fn is_object_or_null(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "32")]
    pub fn is_object_or_null(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "64")]
    pub fn to_object_or_null(&self) -> *mut JSObject {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "32")]
    pub fn to_object_or_null(&self) -> *mut JSObject {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "64")]
    pub fn to_private(&self) -> *const c_void {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "32")]
    pub fn to_private(&self) -> *const c_void {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "64")]
    pub fn is_gcthing(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "32")]
    pub fn is_gcthing(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "64")]
    pub fn to_gcthing(&self) -> *mut c_void {
        unimplemented!();
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "32")]
    pub fn to_gcthing(&self) -> *mut c_void {
        unimplemented!();
    }

    #[inline(always)]
    pub fn is_markable(&self) -> bool {
        unimplemented!();
    }

    #[inline(always)]
    pub fn trace_kind(&self) -> TraceKind {
        unimplemented!();
    }
}
