/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Conversions of Rust values to and from `JSVal`.
//!
//! | IDL type                | Type                             |
//! |-------------------------|----------------------------------|
//! | any                     | `JSVal`                          |
//! | boolean                 | `bool`                           |
//! | byte                    | `i8`                             |
//! | octet                   | `u8`                             |
//! | short                   | `i16`                            |
//! | unsigned short          | `u16`                            |
//! | long                    | `i32`                            |
//! | unsigned long           | `u32`                            |
//! | long long               | `i64`                            |
//! | unsigned long long      | `u64`                            |
//! | unrestricted float      | `f32`                            |
//! | float                   | `Finite<f32>`                    |
//! | unrestricted double     | `f64`                            |
//! | double                  | `Finite<f64>`                    |
//! | USVString               | `String`                         |
//! | object                  | `*mut JSObject`                  |
//! | nullable types          | `Option<T>`                      |
//! | sequences               | `Vec<T>`                         |

#![deny(missing_docs)]

use jsapi::{JSContext, JSObject, JSString, HandleValue, MutableHandleValue};
use jsapi::{ForOfIterator};
use jsval::{JSVal};
use rust::{ToBoolean, ToNumber, ToUint16, ToInt32, ToUint32, ToInt64, ToUint64, ToString};
use libc;
use num_traits::{Bounded, Zero};
use std::rc::Rc;
use std::{ptr, slice};

trait As<O>: Copy {
    fn cast(self) -> O;
}

macro_rules! impl_as {
    ($I:ty, $O:ty) => (
        impl As<$O> for $I {
            fn cast(self) -> $O {
                self as $O
            }
        }
    )
}

impl_as!(f64, u8);
impl_as!(f64, u16);
impl_as!(f64, u32);
impl_as!(f64, u64);
impl_as!(f64, i8);
impl_as!(f64, i16);
impl_as!(f64, i32);
impl_as!(f64, i64);

impl_as!(u8, f64);
impl_as!(u16, f64);
impl_as!(u32, f64);
impl_as!(u64, f64);
impl_as!(i8, f64);
impl_as!(i16, f64);
impl_as!(i32, f64);
impl_as!(i64, f64);

impl_as!(i32, i8);
impl_as!(i32, u8);
impl_as!(i32, i16);
impl_as!(u16, u16);
impl_as!(i32, i32);
impl_as!(u32, u32);
impl_as!(i64, i64);
impl_as!(u64, u64);

/// A trait to convert Rust types to `JSVal`s.
pub trait ToJSValConvertible {
    /// Convert `self` to a `JSVal`. JSAPI failure causes a panic.
    #[inline]
    unsafe fn to_jsval(&self, cx: *mut JSContext, rval: MutableHandleValue);
}

/// A trait to convert `JSVal`s to Rust types.
pub trait FromJSValConvertible: Sized {
    /// Optional configurable behaviour switch; use () for no configuration.
    type Config;
    /// Convert `val` to type `Self`.
    /// Optional configuration of type `T` can be passed as the `option`
    /// argument.
    /// If it returns `Err(())`, a JSAPI exception is pending.
    unsafe fn from_jsval(cx: *mut JSContext,
                         val: HandleValue,
                         option: Self::Config)
                         -> Result<Self, ()>;
}

/// Behavior for converting out-of-range integers.
#[derive(PartialEq, Eq, Clone)]
pub enum ConversionBehavior {
    /// Wrap into the integer's range.
    Default,
    /// Throw an exception.
    EnforceRange,
    /// Clamp into the integer's range.
    Clamp,
}

/// Try to cast the number to a smaller type, but
/// if it doesn't fit, it will return an error.
unsafe fn enforce_range<D>(cx: *mut JSContext, d: f64) -> Result<D, ()>
    where D: Bounded + As<f64>,
          f64: As<D>
{
    unimplemented!();
}

/// Try to cast the number to a smaller type, but if it doesn't fit,
/// round it to the MAX or MIN of the source type before casting it to
/// the destination type.
fn clamp_to<D>(d: f64) -> D
    where D: Bounded + As<f64> + Zero,
          f64: As<D>
{
    unimplemented!();
}

// https://heycam.github.io/webidl/#es-void
impl ToJSValConvertible for () {
    #[inline]
    unsafe fn to_jsval(&self, _cx: *mut JSContext, rval: MutableHandleValue) {
        unimplemented!();
    }
}

impl ToJSValConvertible for JSVal {
    #[inline]
    unsafe fn to_jsval(&self, cx: *mut JSContext, rval: MutableHandleValue) {
        unimplemented!();
    }
}

impl FromJSValConvertible for JSVal {
    type Config = ();
    unsafe fn from_jsval(_cx: *mut JSContext,
                         value: HandleValue,
                         _option: ())
                         -> Result<JSVal, ()> {
        unimplemented!();
    }
}

impl ToJSValConvertible for HandleValue {
    #[inline]
    unsafe fn to_jsval(&self, cx: *mut JSContext, rval: MutableHandleValue) {
        unimplemented!();
    }
}

#[inline]
unsafe fn convert_int_from_jsval<T, M>(cx: *mut JSContext, value: HandleValue,
                                       option: ConversionBehavior,
                                       convert_fn: unsafe fn(*mut JSContext, HandleValue) -> Result<M, ()>)
                                       -> Result<T, ()>
    where T: Bounded + Zero + As<f64>,
          M: Zero + As<T>,
          f64: As<T>
{
        unimplemented!();
}

// https://heycam.github.io/webidl/#es-boolean
impl ToJSValConvertible for bool {
    #[inline]
    unsafe fn to_jsval(&self, _cx: *mut JSContext, rval: MutableHandleValue) {
        unimplemented!();
    }
}

// https://heycam.github.io/webidl/#es-boolean
impl FromJSValConvertible for bool {
    type Config = ();
    unsafe fn from_jsval(_cx: *mut JSContext, val: HandleValue, _option: ()) -> Result<bool, ()> {
        unimplemented!();
    }
}

// https://heycam.github.io/webidl/#es-byte
impl ToJSValConvertible for i8 {
    #[inline]
    unsafe fn to_jsval(&self, _cx: *mut JSContext, rval: MutableHandleValue) {
        unimplemented!();
    }
}

// https://heycam.github.io/webidl/#es-byte
impl FromJSValConvertible for i8 {
    type Config = ConversionBehavior;
    unsafe fn from_jsval(cx: *mut JSContext,
                         val: HandleValue,
                         option: ConversionBehavior)
                         -> Result<i8, ()> {
        unimplemented!();
    }
}

// https://heycam.github.io/webidl/#es-octet
impl ToJSValConvertible for u8 {
    #[inline]
    unsafe fn to_jsval(&self, _cx: *mut JSContext, rval: MutableHandleValue) {
        unimplemented!();
    }
}

// https://heycam.github.io/webidl/#es-octet
impl FromJSValConvertible for u8 {
    type Config = ConversionBehavior;
    unsafe fn from_jsval(cx: *mut JSContext,
                         val: HandleValue,
                         option: ConversionBehavior)
                         -> Result<u8, ()> {
        unimplemented!();
    }
}

// https://heycam.github.io/webidl/#es-short
impl ToJSValConvertible for i16 {
    #[inline]
    unsafe fn to_jsval(&self, _cx: *mut JSContext, rval: MutableHandleValue) {
        unimplemented!();
    }
}

// https://heycam.github.io/webidl/#es-short
impl FromJSValConvertible for i16 {
    type Config = ConversionBehavior;
    unsafe fn from_jsval(cx: *mut JSContext,
                         val: HandleValue,
                         option: ConversionBehavior)
                         -> Result<i16, ()> {
        unimplemented!();
    }
}

// https://heycam.github.io/webidl/#es-unsigned-short
impl ToJSValConvertible for u16 {
    #[inline]
    unsafe fn to_jsval(&self, _cx: *mut JSContext, rval: MutableHandleValue) {
        unimplemented!();
    }
}

// https://heycam.github.io/webidl/#es-unsigned-short
impl FromJSValConvertible for u16 {
    type Config = ConversionBehavior;
    unsafe fn from_jsval(cx: *mut JSContext,
                         val: HandleValue,
                         option: ConversionBehavior)
                         -> Result<u16, ()> {
        unimplemented!();
    }
}

// https://heycam.github.io/webidl/#es-long
impl ToJSValConvertible for i32 {
    #[inline]
    unsafe fn to_jsval(&self, _cx: *mut JSContext, rval: MutableHandleValue) {
        unimplemented!();
    }
}

// https://heycam.github.io/webidl/#es-long
impl FromJSValConvertible for i32 {
    type Config = ConversionBehavior;
    unsafe fn from_jsval(cx: *mut JSContext,
                         val: HandleValue,
                         option: ConversionBehavior)
                         -> Result<i32, ()> {
        unimplemented!();
    }
}

// https://heycam.github.io/webidl/#es-unsigned-long
impl ToJSValConvertible for u32 {
    #[inline]
    unsafe fn to_jsval(&self, _cx: *mut JSContext, rval: MutableHandleValue) {
        unimplemented!();
    }
}

// https://heycam.github.io/webidl/#es-unsigned-long
impl FromJSValConvertible for u32 {
    type Config = ConversionBehavior;
    unsafe fn from_jsval(cx: *mut JSContext,
                         val: HandleValue,
                         option: ConversionBehavior)
                         -> Result<u32, ()> {
        unimplemented!();
    }
}

// https://heycam.github.io/webidl/#es-long-long
impl ToJSValConvertible for i64 {
    #[inline]
    unsafe fn to_jsval(&self, _cx: *mut JSContext, rval: MutableHandleValue) {
        unimplemented!();
    }
}

// https://heycam.github.io/webidl/#es-long-long
impl FromJSValConvertible for i64 {
    type Config = ConversionBehavior;
    unsafe fn from_jsval(cx: *mut JSContext,
                         val: HandleValue,
                         option: ConversionBehavior)
                         -> Result<i64, ()> {
        unimplemented!();
    }
}

// https://heycam.github.io/webidl/#es-unsigned-long-long
impl ToJSValConvertible for u64 {
    #[inline]
    unsafe fn to_jsval(&self, _cx: *mut JSContext, rval: MutableHandleValue) {
        unimplemented!();
    }
}

// https://heycam.github.io/webidl/#es-unsigned-long-long
impl FromJSValConvertible for u64 {
    type Config = ConversionBehavior;
    unsafe fn from_jsval(cx: *mut JSContext,
                         val: HandleValue,
                         option: ConversionBehavior)
                         -> Result<u64, ()> {
        unimplemented!();
    }
}

// https://heycam.github.io/webidl/#es-float
impl ToJSValConvertible for f32 {
    #[inline]
    unsafe fn to_jsval(&self, _cx: *mut JSContext, rval: MutableHandleValue) {
        unimplemented!();
    }
}

// https://heycam.github.io/webidl/#es-float
impl FromJSValConvertible for f32 {
    type Config = ();
    unsafe fn from_jsval(cx: *mut JSContext, val: HandleValue, _option: ()) -> Result<f32, ()> {
        unimplemented!();
    }
}

// https://heycam.github.io/webidl/#es-double
impl ToJSValConvertible for f64 {
    #[inline]
    unsafe fn to_jsval(&self, _cx: *mut JSContext, rval: MutableHandleValue) {
        unimplemented!();
    }
}

// https://heycam.github.io/webidl/#es-double
impl FromJSValConvertible for f64 {
    type Config = ();
    unsafe fn from_jsval(cx: *mut JSContext, val: HandleValue, _option: ()) -> Result<f64, ()> {
        unimplemented!();
    }
}

/// Converts a `JSString`, encoded in "Latin1" (i.e. U+0000-U+00FF encoded as 0x00-0xFF) into a
/// `String`.
pub unsafe fn latin1_to_string(cx: *mut JSContext, s: *mut JSString) -> String {
        unimplemented!();
}

// https://heycam.github.io/webidl/#es-USVString
impl ToJSValConvertible for str {
    #[inline]
    unsafe fn to_jsval(&self, cx: *mut JSContext, rval: MutableHandleValue) {
        unimplemented!();
    }
}

// https://heycam.github.io/webidl/#es-USVString
impl ToJSValConvertible for String {
    #[inline]
    unsafe fn to_jsval(&self, cx: *mut JSContext, rval: MutableHandleValue) {
        unimplemented!();
    }
}

// https://heycam.github.io/webidl/#es-USVString
impl FromJSValConvertible for String {
    type Config = ();
    unsafe fn from_jsval(cx: *mut JSContext, value: HandleValue, _: ()) -> Result<String, ()> {
        unimplemented!();
    }
}

impl<T: ToJSValConvertible> ToJSValConvertible for Option<T> {
    #[inline]
    unsafe fn to_jsval(&self, cx: *mut JSContext, rval: MutableHandleValue) {
        unimplemented!();
    }
}

impl<T: ToJSValConvertible> ToJSValConvertible for Rc<T> {
    #[inline]
    unsafe fn to_jsval(&self, cx: *mut JSContext, rval: MutableHandleValue) {
        unimplemented!();
    }
}

impl<T: FromJSValConvertible> FromJSValConvertible for Option<T> {
    type Config = T::Config;
    unsafe fn from_jsval(cx: *mut JSContext,
                         value: HandleValue,
                         option: T::Config)
                         -> Result<Option<T>, ()> {
        unimplemented!();
    }
}

// https://heycam.github.io/webidl/#es-sequence
impl<T: ToJSValConvertible> ToJSValConvertible for Vec<T> {
    #[inline]
    unsafe fn to_jsval(&self, cx: *mut JSContext, rval: MutableHandleValue) {
        unimplemented!();
    }
}

/// Rooting guard for the iterator field of ForOfIterator.
/// Behaves like RootedGuard (roots on creation, unroots on drop),
/// but borrows and allows access to the whole ForOfIterator, so
/// that methods on ForOfIterator can still be used through it.
struct ForOfIteratorGuard<'a> {
    root: &'a mut ForOfIterator
}

impl<'a> ForOfIteratorGuard<'a> {
    fn new(cx: *mut JSContext, root: &'a mut ForOfIterator) -> Self {
        unimplemented!();
    }
}

impl<'a> Drop for ForOfIteratorGuard<'a> {
    fn drop(&mut self) {
        unimplemented!();
    }
}

impl<C: Clone, T: FromJSValConvertible<Config=C>> FromJSValConvertible for Vec<T> {
    type Config = C;

    unsafe fn from_jsval(cx: *mut JSContext,
                         value: HandleValue,
                         option: C)
                         -> Result<Vec<T>, ()> {
        unimplemented!();
    }
}

// https://heycam.github.io/webidl/#es-object
impl ToJSValConvertible for *mut JSObject {
    #[inline]
    unsafe fn to_jsval(&self, cx: *mut JSContext, rval: MutableHandleValue) {
        unimplemented!();
    }
}
