use jsapi::*;

pub fn InvokeGetOwnPropertyDescriptor(handler: *const ::libc::c_void,
                                      cx: *mut JSContext,
                                      proxy: HandleObject, id: HandleId,
                                      desc:
                                          MutableHandle<PropertyDescriptor>)
 -> bool {
    unimplemented!();
}
pub fn RUST_FUNCTION_VALUE_TO_JITINFO(v: Value) -> *const JSJitInfo {
    unimplemented!();
}
pub fn CallJitGetterOp(info: *const JSJitInfo, cx: *mut JSContext,
                       thisObj: HandleObject,
                       specializedThis: *mut ::libc::c_void, argc: u32,
                       vp: *mut Value) -> bool {
    unimplemented!();
}
pub fn CallJitSetterOp(info: *const JSJitInfo, cx: *mut JSContext,
                       thisObj: HandleObject,
                       specializedThis: *mut ::libc::c_void, argc: u32,
                       vp: *mut Value) -> bool {
    unimplemented!();
}
pub fn CallJitMethodOp(info: *const JSJitInfo, cx: *mut JSContext,
                       thisObj: HandleObject,
                       specializedThis: *mut ::libc::c_void, argc: u32,
                       vp: *mut Value) -> bool {
    unimplemented!();
}
pub fn CreateProxyHandler(aExtra: *const ::libc::c_void)
 -> *const ::libc::c_void {
    unimplemented!();
}
pub fn GetCrossCompartmentWrapper() -> *const ::libc::c_void {
    unimplemented!();
}
pub fn NewProxyObject(aCx: *mut JSContext,
                      aHandler: *const ::libc::c_void, aPriv: HandleValue,
                      proto: *mut JSObject, parent: *mut JSObject,
                      call: *mut JSObject, construct: *mut JSObject)
 -> *mut JSObject {
    unimplemented!();
}
pub fn WrapperNew(aCx: *mut JSContext, aObj: HandleObject,
                  aHandler: *const ::libc::c_void, aClass: *const JSClass,
                  aSingleton: bool)
 -> *mut JSObject {
    unimplemented!();
}
pub fn NewWindowProxy(aCx: *mut JSContext, aObj: HandleObject)
 -> *mut JSObject {
    unimplemented!();
}
pub fn GetWindowProxyClass() -> *const Class {
    unimplemented!();
}
pub fn GetProxyExtra(obj: *mut JSObject, slot: u32) -> Value {
    unimplemented!();
}
pub fn GetProxyPrivate(obj: *mut JSObject) -> Value {
    unimplemented!();
}
pub fn SetProxyExtra(obj: *mut JSObject, slot: u32, val: *const Value) {
    unimplemented!();
}
pub fn RUST_JSID_IS_INT(id: HandleId) -> bool {
    unimplemented!();
}
pub fn RUST_JSID_TO_INT(id: HandleId) -> i32 {
    unimplemented!();
}
pub fn int_to_jsid(i: i32) -> jsid {
    unimplemented!();
}
pub fn RUST_JSID_IS_STRING(id: HandleId) -> bool {
    unimplemented!();
}
pub fn RUST_JSID_TO_STRING(id: HandleId) -> *mut JSString {
    unimplemented!();
}
pub fn RUST_SYMBOL_TO_JSID(sym: *mut Symbol) -> jsid {
    unimplemented!();
}
pub fn IsProxyHandlerFamily(obj: *mut JSObject) -> u8 {
    unimplemented!();
}
pub fn GetProxyHandlerExtra(obj: *mut JSObject) -> *const ::libc::c_void {
    unimplemented!();
}
pub fn GetProxyHandler(obj: *mut JSObject) -> *const ::libc::c_void {
    unimplemented!();
}
pub fn IsWrapper(obj: *mut JSObject) -> bool {
    unimplemented!();
}
pub fn UnwrapObject(obj: *mut JSObject, stopAtOuter: u8) -> *mut JSObject {
    unimplemented!();
}
pub fn UncheckedUnwrapObject(obj: *mut JSObject, stopAtOuter: u8) -> *mut JSObject {
    unimplemented!();
}
pub fn AppendToAutoIdVector(v: *mut AutoIdVector, id: jsid) -> bool {
    unimplemented!();
}
pub fn CollectServoSizes(rt: *mut JSRuntime, sizes: *mut ServoSizes) -> bool {
    unimplemented!();
}
pub fn CallValueTracer(trc: *mut JSTracer, valuep: *mut Heap<Value>,
                       name: *const ::libc::c_char) {
    unimplemented!();
}
pub fn CallObjectTracer(trc: *mut JSTracer,
                        objp: *mut Heap<*mut JSObject>,
                        name: *const ::libc::c_char) {
    unimplemented!();
}
pub fn CallUnbarrieredObjectTracer(trc: *mut JSTracer,
                                   objp: *mut *mut JSObject,
                                   name: *const ::libc::c_char) {
    unimplemented!();
}

