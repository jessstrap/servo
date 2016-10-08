
//! Rust wrappers around the raw JS apis

use libc::{size_t, c_uint, c_char};
use heapsize::HeapSizeOf;
use std::char;
use std::ffi;
use std::ptr;
use std::slice;
use std::mem;
use std::u32;
use std::default::Default;
use std::ops::{Deref, DerefMut};
use std::cell::UnsafeCell;
use std::marker::PhantomData;
use jsapi;
use jsapi::{JSContext, JSRuntime, JSObject, JSFlatString, JSFunction, JSString, Symbol, JSScript, jsid, Value};
use jsapi::{ReadOnlyCompileOptions};
use jsapi::{Heap};
use jsapi::{Rooted, Handle, MutableHandle, RootedBase};
use jsapi::{MutableHandleValue, HandleValue, HandleObject};
use jsapi::AutoObjectVector;
use jsapi::{JSAutoCompartment};
use jsapi::{JSJitMethodCallArgs, JSJitGetterCallArgs, JSJitSetterCallArgs, CallArgs};
use jsapi::CompartmentOptions;
use jsapi::JSFunctionSpec;
use jsapi::JSNativeWrapper;
use jsapi::JSPropertySpec;
use jsapi::JS_SetNativeStackQuota;
use jsval::UndefinedValue;

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/. */

// From Gecko:
// Our "default" stack is what we use in configurations where we don't have a compelling reason to
// do things differently. This is effectively 1MB on 64-bit platforms.
//const STACK_QUOTA: usize = 128 * 8 * 1024;

// From Gecko:
// The JS engine permits us to set different stack limits for system code,
// trusted script, and untrusted script. We have tests that ensure that
// we can always execute 10 "heavy" (eval+with) stack frames deeper in
// privileged code. Our stack sizes vary greatly in different configurations,
// so satisfying those tests requires some care. Manual measurements of the
// number of heavy stack frames achievable gives us the following rough data,
// ordered by the effective categories in which they are grouped in the
// JS_SetNativeStackQuota call (which predates this analysis).
//
// (NB: These numbers may have drifted recently - see bug 938429)
// OSX 64-bit Debug: 7MB stack, 636 stack frames => ~11.3k per stack frame
// OSX64 Opt: 7MB stack, 2440 stack frames => ~3k per stack frame
//
// Linux 32-bit Debug: 2MB stack, 426 stack frames => ~4.8k per stack frame
// Linux 64-bit Debug: 4MB stack, 455 stack frames => ~9.0k per stack frame
//
// Windows (Opt+Debug): 900K stack, 235 stack frames => ~3.4k per stack frame
//
// Linux 32-bit Opt: 1MB stack, 272 stack frames => ~3.8k per stack frame
// Linux 64-bit Opt: 2MB stack, 316 stack frames => ~6.5k per stack frame
//
// We tune the trusted/untrusted quotas for each configuration to achieve our
// invariants while attempting to minimize overhead. In contrast, our buffer
// between system code and trusted script is a very unscientific 10k.
//const SYSTEM_CODE_BUFFER: usize = 10 * 1024;

// Gecko's value on 64-bit.
//const TRUSTED_SCRIPT_BUFFER: usize = 8 * 12800;




// ___________________________________________________________________________
// Rooting API for standard JS things

pub trait RootKind {
    #[inline(always)]
    fn rootKind() -> jsapi::RootKind;
}

impl RootKind for *mut JSObject {
    #[inline(always)]
    fn rootKind() -> jsapi::RootKind { unimplemented!(); }
}

impl RootKind for *mut JSFlatString {
    #[inline(always)]
    fn rootKind() -> jsapi::RootKind { unimplemented!(); }
}

impl RootKind for *mut JSFunction {
    #[inline(always)]
    fn rootKind() -> jsapi::RootKind { unimplemented!(); }
}

impl RootKind for *mut JSString {
    #[inline(always)]
    fn rootKind() -> jsapi::RootKind { unimplemented!();}
}

impl RootKind for *mut Symbol {
    #[inline(always)]
    fn rootKind() -> jsapi::RootKind { unimplemented!(); }
}

impl RootKind for *mut JSScript {
    #[inline(always)]
    fn rootKind() -> jsapi::RootKind { unimplemented!(); }
}

impl RootKind for jsid {
    #[inline(always)]
    fn rootKind() -> jsapi::RootKind { unimplemented!();}
}

impl RootKind for Value {
    #[inline(always)]
    fn rootKind() -> jsapi::RootKind { unimplemented!(); }
}

impl<T> Rooted<T> {
    pub fn new_unrooted(initial: T) -> Rooted<T> {
        Rooted {
            //_base: RootedBase { _phantom0: PhantomData },
            stack: ptr::null_mut(),
            prev: ptr::null_mut(),
            ptr: initial,
        }
    }

    pub unsafe fn add_to_root_stack(&mut self, cx: *mut JSContext) where T: RootKind {
        unimplemented!();
    }

    pub unsafe fn remove_from_root_stack(&mut self) {
        unimplemented!();
    }
}

/// Rust API for keeping a Rooted value in the context's root stack.
/// Example usage: `rooted!(in(cx) let x = UndefinedValue());`.
/// `RootedGuard::new` also works, but the macro is preferred.
pub struct RootedGuard<'a, T: 'a> {
    root: &'a mut Rooted<T>
}

impl<'a, T> RootedGuard<'a, T> {
    pub fn new(cx: *mut JSContext, root: &'a mut Rooted<T>) -> Self where T: RootKind {
        unimplemented!();
    }

    pub fn handle(&self) -> Handle<T> {
        unimplemented!();
    }

    pub fn handle_mut(&mut self) -> MutableHandle<T> {
        unimplemented!();
    }

    pub fn get(&self) -> T where T: Copy {
        unimplemented!();
    }

    pub fn set(&mut self, v: T) {
        unimplemented!();
    }
}

impl<'a, T> Deref for RootedGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unimplemented!();
    }
}

impl<'a, T> DerefMut for RootedGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unimplemented!();
    }
}

impl<'a, T> Drop for RootedGuard<'a, T> {
    fn drop(&mut self) {
        unimplemented!();
    }
}

#[macro_export]
macro_rules! rooted {
    (in($cx:expr) let $name:ident = $init:expr) => {
        let mut __root = $crate::jsapi::Rooted::new_unrooted($init);
        let $name = $crate::rust::RootedGuard::new($cx, &mut __root);
    };
    (in($cx:expr) let mut $name:ident = $init:expr) => {
        let mut __root = $crate::jsapi::Rooted::new_unrooted($init);
        let mut $name = $crate::rust::RootedGuard::new($cx, &mut __root);
    }
}

impl<T> Handle<T> {
    pub fn get(&self) -> T
        where T: Copy
    {
        unimplemented!();
    }

    pub unsafe fn from_marked_location(ptr: *const T) -> Handle<T> {
        unimplemented!();
    }
}

impl<T> Deref for Handle<T> {
    type Target = T;

    fn deref<'a>(&'a self) -> &'a T {
        unimplemented!();
    }
}

impl<T> MutableHandle<T> {
    pub unsafe fn from_marked_location(ptr: *mut T) -> MutableHandle<T> {
        unimplemented!();
    }

    pub fn handle(&self) -> Handle<T> {
        unimplemented!();
    }

    pub fn get(&self) -> T
        where T: Copy
    {
        unimplemented!();
    }

    pub fn set(&self, v: T)
        where T: Copy
    {
        unimplemented!();
    }
}

impl<T> Deref for MutableHandle<T> {
    type Target = T;

    fn deref<'a>(&'a self) -> &'a T {
        unimplemented!();
    }
}

impl<T> DerefMut for MutableHandle<T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        unimplemented!();
    }
}





// ___________________________________________________________________________
// Fast inline converters

#[inline]
pub unsafe fn ToBoolean(v: HandleValue) -> bool {
        unimplemented!();
}

#[inline]
pub unsafe fn ToNumber(cx: *mut JSContext, v: HandleValue) -> Result<f64, ()> {
        unimplemented!();
}

#[inline]
unsafe fn convert_from_int32<T: Default + Copy>(
    cx: *mut JSContext,
    v: HandleValue,
    conv_fn: unsafe extern "C" fn(*mut JSContext, HandleValue, *mut T) -> bool)
        -> Result<T, ()> {
        unimplemented!();
}

#[inline]
pub unsafe fn ToInt32(cx: *mut JSContext, v: HandleValue) -> Result<i32, ()> {
        unimplemented!();
}

#[inline]
pub unsafe fn ToUint32(cx: *mut JSContext, v: HandleValue) -> Result<u32, ()> {
        unimplemented!();
}

#[inline]
pub unsafe fn ToUint16(cx: *mut JSContext, v: HandleValue) -> Result<u16, ()> {
        unimplemented!();
}

#[inline]
pub unsafe fn ToInt64(cx: *mut JSContext, v: HandleValue) -> Result<i64, ()> {
        unimplemented!();
}

#[inline]
pub unsafe fn ToUint64(cx: *mut JSContext, v: HandleValue) -> Result<u64, ()> {
        unimplemented!();
}

#[inline]
pub unsafe fn ToString(cx: *mut JSContext, v: HandleValue) -> *mut JSString {
        unimplemented!();
}




pub trait GCMethods<T> {
    unsafe fn initial() -> T;
    unsafe fn is_dropped(self) -> bool;
    unsafe fn post_barrier(v: *mut T, prev: T, next: T);
}

impl GCMethods<jsid> for jsid {
    unsafe fn initial() -> jsid {
        unimplemented!(); }
    unsafe fn is_dropped(self) -> bool { 
        unimplemented!(); }
    unsafe fn post_barrier(_: *mut jsid, _: jsid, _: jsid) {
        unimplemented!();}
}

impl GCMethods<*mut JSObject> for *mut JSObject {
    unsafe fn initial() -> *mut JSObject { 
        unimplemented!(); }
    unsafe fn is_dropped(self) -> bool { 
        unimplemented!(); }
    unsafe fn post_barrier(v: *mut *mut JSObject,
                           prev: *mut JSObject, next: *mut JSObject) {
        unimplemented!();
    }
}

impl GCMethods<*mut JSString> for *mut JSString {
    unsafe fn initial() -> *mut JSString { 
        unimplemented!(); }
    unsafe fn is_dropped(self) -> bool { 
        unimplemented!();}
    unsafe fn post_barrier(_: *mut *mut JSString, _: *mut JSString, _: *mut JSString) {
        unimplemented!();}
}

impl GCMethods<*mut JSScript> for *mut JSScript {
    unsafe fn initial() -> *mut JSScript { 
        unimplemented!(); }
    unsafe fn is_dropped(self) -> bool {
        unimplemented!(); }
    unsafe fn post_barrier(_: *mut *mut JSScript, _: *mut JSScript, _: *mut JSScript) { 
        unimplemented!();}
}

impl GCMethods<*mut JSFunction> for *mut JSFunction {
    unsafe fn initial() -> *mut JSFunction { 
        unimplemented!(); }
    unsafe fn is_dropped(self) -> bool { 
        unimplemented!();}
    unsafe fn post_barrier(v: *mut *mut JSFunction,
                           prev: *mut JSFunction, next: *mut JSFunction) {
        unimplemented!();
    }
}

impl GCMethods<Value> for Value {
    unsafe fn initial() -> Value { 
        unimplemented!(); }
    unsafe fn is_dropped(self) -> bool { 
        unimplemented!(); }
    unsafe fn post_barrier(v: *mut Value, prev: Value, next: Value) {
        unimplemented!();
    }
}

impl<T: GCMethods<T> + Copy> Heap<T> {
    pub fn set(&mut self, v: T) {
        unimplemented!();
    }

    pub fn get(&self) -> T {
        unimplemented!();
    }

    pub fn get_unsafe(&self) -> *mut T {
        unimplemented!();
    }

    pub fn handle(&self) -> Handle<T> {
        unimplemented!();
    }
}



// ___________________________________________________________________________
// friendly Rustic API to runtimes

/// A wrapper for the `JSRuntime` and `JSContext` structures in SpiderMonkey.
pub struct Runtime {
    rt: *mut JSRuntime,
    cx: *mut JSContext,
}

impl Runtime {
    /// Creates a new `JSRuntime` and `JSContext`.
    pub fn new() -> Runtime {
        unimplemented!();
    }

    /// Returns the `JSRuntime` object.
    pub fn rt(&self) -> *mut JSRuntime {
        unimplemented!();
    }

    /// Returns the `JSContext` object.
    pub fn cx(&self) -> *mut JSContext {
        unimplemented!();
    }

    pub fn evaluate_script(&self, glob: HandleObject, script: &str, filename: &str,
                           line_num: u32, rval: MutableHandleValue)
                    -> Result<(),()> {
        unimplemented!();
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        unimplemented!();
    }
}

// This is measured through `glue::CollectServoSizes`.
impl HeapSizeOf for Runtime {
    fn heap_size_of_children(&self) -> usize {
        unimplemented!();
    }
}



trait ToResult {
    fn to_result(self) -> Result<(), ()>;
}

impl ToResult for bool {
    fn to_result(self) -> Result<(), ()> {
        if self {
            Ok(())
        } else {
            Err(())
        }
    }
}





impl HandleValue {
    pub fn null() -> HandleValue {
        unimplemented!();
    }

    pub fn undefined() -> HandleValue {
        unimplemented!();
    }
}

//const ConstNullValue: *mut JSObject = 0 as *mut JSObject;

impl HandleObject {
    pub fn null() -> HandleObject {
        unimplemented!();
    }
}

impl Default for jsid {
    fn default() -> jsid { 
        unimplemented!();}
}

impl Default for Value {
    fn default() -> Value { 
        unimplemented!(); }
}

impl Default for CompartmentOptions {
    fn default() -> Self { unsafe { 
        unimplemented!(); } }
}

//const ChunkShift: usize = 20;
//const ChunkSize: usize = 1 << ChunkShift;

//#[cfg(target_pointer_width = "32")]
//const ChunkLocationOffset: usize = ChunkSize - 2 * 4 - 8;


impl Default for Heap<*mut JSObject> {
    fn default() -> Heap<*mut JSObject> {
        unimplemented!();
    }
}

impl Default for Heap<Value> {
    fn default() -> Heap<Value> {
        unimplemented!();
    }
}

impl<T: GCMethods<T> + Copy> Drop for Heap<T> {
    fn drop(&mut self) {
        unimplemented!();
    }
}


// ___________________________________________________________________________
// Implementations for various things in jsapi.rs

impl JSAutoCompartment {
    pub fn new(cx: *mut JSContext, target: *mut JSObject) -> JSAutoCompartment {
        unimplemented!();
    }
}

impl Drop for JSAutoCompartment {
    fn drop(&mut self) {
        unimplemented!();
    }
}

impl JSJitMethodCallArgs {
    #[inline]
    pub fn get(&self, i: u32) -> HandleValue {
        unimplemented!();
    }

    #[inline]
    pub fn index(&self, i: u32) -> HandleValue {
        unimplemented!();
    }

    #[inline]
    pub fn index_mut(&self, i: u32) -> MutableHandleValue {
        unimplemented!();
    }

    #[inline]
    pub fn rval(&self) -> MutableHandleValue {
        unimplemented!();
    }
}

// XXX need to hack up bindgen to convert this better so we don't have
//     to duplicate so much code here
impl CallArgs {
    #[inline]
    pub unsafe fn from_vp(vp: *mut Value, argc: u32) -> CallArgs {
        unimplemented!();
    }

    #[inline]
    pub fn index(&self, i: u32) -> HandleValue {
        unimplemented!();
    }

    #[inline]
    pub fn index_mut(&self, i: u32) -> MutableHandleValue {
        unimplemented!();
    }

    #[inline]
    pub fn get(&self, i: u32) -> HandleValue {
        unimplemented!();
    }

    #[inline]
    pub fn rval(&self) -> MutableHandleValue {
        unimplemented!();
    }

    #[inline]
    pub fn thisv(&self) -> HandleValue {
        unimplemented!();
    }
}

impl JSJitGetterCallArgs {
    #[inline]
    pub fn rval(&self) -> MutableHandleValue {
        unimplemented!();
    }
}

impl JSJitSetterCallArgs {
    #[inline]
    pub fn get(&self, i: u32) -> HandleValue {
        unimplemented!();
    }
}

// ___________________________________________________________________________
// Wrappers around things in jsglue.cpp

pub struct AutoObjectVectorWrapper {
    pub ptr: *mut AutoObjectVector
}

impl AutoObjectVectorWrapper {
    pub fn new(cx: *mut JSContext) -> AutoObjectVectorWrapper {
        unimplemented!();
    }

    pub fn append(&self, obj: *mut JSObject) -> bool {
        unimplemented!();
    }
}

impl Drop for AutoObjectVectorWrapper {
    fn drop(&mut self) {
        unimplemented!();
    }
}

pub struct CompileOptionsWrapper {
    pub ptr: *mut ReadOnlyCompileOptions
}

impl CompileOptionsWrapper {
    pub fn new(cx: *mut JSContext, file: *const ::libc::c_char, line: c_uint) -> CompileOptionsWrapper {
        unimplemented!();
    }
}

impl Drop for CompileOptionsWrapper {
    fn drop(&mut self) {
        unimplemented!();
    }
}

impl JSNativeWrapper {
    fn is_zeroed(&self) -> bool {
        unimplemented!();
    }
}

/// Defines methods on `obj`. The last entry of `methods` must contain zeroed
/// memory.
///
/// # Failures
///
/// Returns `Err` on JSAPI failure.
///
/// # Panics
///
/// Panics if the last entry of `methods` does not contain zeroed memory.
///
/// # Safety
///
/// - `cx` must be valid.
/// - This function calls into unaudited C++ code.
pub unsafe fn define_methods(cx: *mut JSContext, obj: HandleObject,
                             methods: &'static [JSFunctionSpec])
                             -> Result<(), ()> {
        unimplemented!();
}

/// Defines attributes on `obj`. The last entry of `properties` must contain
/// zeroed memory.
///
/// # Failures
///
/// Returns `Err` on JSAPI failure.
///
/// # Panics
///
/// Panics if the last entry of `properties` does not contain zeroed memory.
///
/// # Safety
///
/// - `cx` must be valid.
/// - This function calls into unaudited C++ code.
pub unsafe fn define_properties(cx: *mut JSContext, obj: HandleObject,
                                properties: &'static [JSPropertySpec])
                                -> Result<(), ()> {
        unimplemented!();
}
