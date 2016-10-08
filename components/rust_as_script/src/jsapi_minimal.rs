
use libc::FILE;
use heapsize::HeapSizeOf;
use rust::GCMethods;

#[repr(C)]
#[unsafe_no_drop_flag]
#[derive(Debug)]
pub struct Heap<T: ::rust::GCMethods<T> + Copy> {
    pub ptr: ::std::cell::UnsafeCell<T>,
}

unsafe impl Sync for JSClass {}

impl ObjectOpResult {
    /// Set this ObjectOpResult to true and return true.
    pub fn succeed(&mut self) -> bool {
        unimplemented!();
    }
}



// This is measured properly by the heap measurement implemented in SpiderMonkey.
impl<T: Copy + GCMethods<T>> HeapSizeOf for Heap<T> {
    fn heap_size_of_children(&self) -> usize {
        unimplemented!();
    }
}

//include!("jsapi_full.rs");

pub enum JSContext { }
pub enum JSFunction { }
pub enum JSObject { }
pub enum JSScript { }
pub enum JSString { }
pub enum Symbol { }
pub enum JSRuntime { }
pub enum JSFlatString { }
pub enum JSCompartment { }
pub type AutoIdVector = AutoVectorRooter<jsid>;
pub type RootedObject = Rooted<*mut JSObject>;
pub type MutableHandleObject = MutableHandle<*mut JSObject>;


#[repr(C)]
#[derive(Debug, Copy)]
pub struct jsid {
    pub asBits: usize,
}
impl ::std::clone::Clone for jsid {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_jsid() {
    assert_eq!(::std::mem::size_of::<jsid>() , 8usize);
    assert_eq!(::std::mem::align_of::<jsid>() , 8usize);
}


/**
 * Similar to a handle, but the underlying storage can be changed. This is
 * useful for outparams.
 *
 * If you want to add additional methods to MutableHandle for a specific
 * specialization, define a MutableHandleBase<T> specialization containing
 * them.
 */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MutableHandle<T> {
    //pub _base: MutableHandleBase<T>,
    pub ptr: *mut T,
}



/**
 * JS::Value is the interface for a single JavaScript Engine value.  A few
 * general notes on JS::Value:
 *
 * - JS::Value has setX() and isX() members for X in
 *
 *     { Int32, Double, String, Symbol, Boolean, Undefined, Null, Object, Magic }
 *
 *   JS::Value also contains toX() for each of the non-singleton types.
 *
 * - Magic is a singleton type whose payload contains either a JSWhyMagic "reason" for
 *   the magic value or a uint32_t value. By providing JSWhyMagic values when
 *   creating and checking for magic values, it is possible to assert, at
 *   runtime, that only magic values with the expected reason flow through a
 *   particular value. For example, if cx->exception has a magic value, the
 *   reason must be JS_GENERATOR_CLOSING.
 *
 * - The JS::Value operations are preferred.  The JSVAL_* operations remain for
 *   compatibility; they may be removed at some point.  These operations mostly
 *   provide similar functionality.  But there are a few key differences.  One
 *   is that JS::Value gives null a separate type.
 *   Also, to help prevent mistakenly boxing a nullable JSObject* as an object,
 *   Value::setObject takes a JSObject&. (Conversely, Value::toObject returns a
 *   JSObject&.)  A convenience member Value::setObjectOrNull is provided.
 *
 * - JSVAL_VOID is the same as the singleton value of the Undefined type.
 *
 * - Note that JS::Value is 8 bytes on 32 and 64-bit architectures. Thus, on
 *   32-bit user code should avoid copying jsval/JS::Value as much as possible,
 *   preferring to pass by const Value&.
 */
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Value {
    //pub data: jsval_layout,
}
impl ::std::clone::Clone for Value {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Value() {
    assert_eq!(::std::mem::size_of::<Value>() , 8usize);
    assert_eq!(::std::mem::align_of::<Value>() , 8usize);
}

/**
 * Local variable of type T whose value is always rooted. This is typically
 * used for local variables, or for non-rooted values being passed to a
 * function that requires a handle, e.g. Foo(Root<T>(cx, x)).
 *
 * If you want to add additional methods to Rooted for a specific
 * specialization, define a RootedBase<T> specialization containing them.
 */
#[repr(C)]
#[unsafe_no_drop_flag]
#[derive(Debug)]
pub struct Rooted<T> {
    //pub _base: RootedBase<T>,
    pub stack: *mut *mut Rooted<*mut ::std::os::raw::c_void>,
    pub prev: *mut Rooted<*mut ::std::os::raw::c_void>,
    pub ptr: T,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RootedBase<T> {
    pub _phantom0: ::std::marker::PhantomData<T>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HandleBase<T> {
    pub _phantom0: ::std::marker::PhantomData<T>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MutableHandleBase<T> {
    pub _phantom0: ::std::marker::PhantomData<T>,
}

/**
 * Reference to a T that has been rooted elsewhere. This is most useful
 * as a parameter type, which guarantees that the T lvalue is properly
 * rooted. See "Move GC Stack Rooting" above.
 *
 * If you want to add additional methods to Handle for a specific
 * specialization, define a HandleBase<T> specialization containing them.
 */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Handle<T> {
    //pub _base: HandleBase<T>,
    pub ptr: *const T,
}
#[repr(C)]
#[unsafe_no_drop_flag]
#[derive(Debug)]
pub struct JSAutoCompartment {
    pub cx_: *mut JSContext,
    pub oldCompartment_: *mut JSCompartment,
}
#[test]
fn bindgen_test_layout_JSAutoCompartment() {
    assert_eq!(::std::mem::size_of::<JSAutoCompartment>() , 16usize);
    assert_eq!(::std::mem::align_of::<JSAutoCompartment>() , 8usize);
}
/**
 * A class, expected to be passed by reference, which represents the CallArgs
 * for a JSJitMethodOp.
 */
#[repr(C)]
#[derive(Debug, Copy)]
pub struct JSJitMethodCallArgs {
    pub _base: CallArgsBase<NoUsedRval>,
}
impl ::std::clone::Clone for JSJitMethodCallArgs {
    fn clone(&self) -> Self { *self }
}
/**
 * A class, expected to be passed by value, which represents the CallArgs for a
 * JSJitGetterOp.
 */
#[repr(C)]
#[derive(Debug, Copy)]
pub struct JSJitGetterCallArgs {
    pub _base: MutableHandleValue,
}
impl ::std::clone::Clone for JSJitGetterCallArgs {
    fn clone(&self) -> Self { *self }
}
/**
 * A class, expected to be passed by value, which represents the CallArgs for a
 * JSJitSetterOp.
 */
#[repr(C)]
#[derive(Debug, Copy)]
pub struct JSJitSetterCallArgs {
    pub _base: MutableHandleValue,
}
impl ::std::clone::Clone for JSJitSetterCallArgs {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CallArgsBase<WantUsedRval> {
    pub _base: WantUsedRval,
    pub argv_: *mut Value,
    pub argc_: ::std::os::raw::c_uint,
    pub constructing_: bool,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct CallArgs {
    pub _base: CallArgsBase<IncludeUsedRval>,
}
impl ::std::clone::Clone for CallArgs {
    fn clone(&self) -> Self { *self }
}
/**
 * CompartmentOptions specifies compartment characteristics: both those that
 * can't be changed on a compartment once it's been created
 * (CompartmentCreationOptions), and those that can be changed on an existing
 * compartment (CompartmentBehaviors).
 */
#[repr(C)]
#[derive(Debug, Copy)]
pub struct CompartmentOptions {
    pub creationOptions_: CompartmentCreationOptions,
    pub behaviors_: CompartmentBehaviors,
}
impl ::std::clone::Clone for CompartmentOptions {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_CompartmentOptions() {
    assert_eq!(::std::mem::size_of::<CompartmentOptions>() , 48usize);
    assert_eq!(::std::mem::align_of::<CompartmentOptions>() , 8usize);
}
/**
 * Wrapper to relace JSNative for JSPropertySpecs and JSFunctionSpecs. This will
 * allow us to pass one JSJitInfo per function with the property/function spec,
 * without additional field overhead.
 */
#[repr(C)]
#[derive(Debug, Copy)]
pub struct JSNativeWrapper {
    pub op: JSNative,
    pub info: *const JSJitInfo,
}
impl ::std::clone::Clone for JSNativeWrapper {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_JSNativeWrapper() {
    assert_eq!(::std::mem::size_of::<JSNativeWrapper>() , 16usize);
    assert_eq!(::std::mem::align_of::<JSNativeWrapper>() , 8usize);
}
/**
 * Description of a property. JS_DefineProperties and JS_InitClass take arrays
 * of these and define many properties at once. JS_PSG, JS_PSGS and JS_PS_END
 * are helper macros for defining such arrays.
 */
#[repr(C)]
#[derive(Debug, Copy)]
pub struct JSPropertySpec {
    pub name: *const ::std::os::raw::c_char,
    pub flags: u8,
    pub getter: JSNativeWrapper,
    pub setter: JSNativeWrapper,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct JSPropertySpec_SelfHostedWrapper {
    pub unused: *mut ::std::os::raw::c_void,
    pub funname: *const ::std::os::raw::c_char,
}
impl ::std::clone::Clone for JSPropertySpec_SelfHostedWrapper {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_JSPropertySpec_SelfHostedWrapper() {
    assert_eq!(::std::mem::size_of::<JSPropertySpec_SelfHostedWrapper>() ,
               16usize);
    assert_eq!(::std::mem::align_of::<JSPropertySpec_SelfHostedWrapper>() ,
               8usize);
}
impl ::std::clone::Clone for JSPropertySpec {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_JSPropertySpec() {
    assert_eq!(::std::mem::size_of::<JSPropertySpec>() , 48usize);
    assert_eq!(::std::mem::align_of::<JSPropertySpec>() , 8usize);
}
/**
 * To define a native function, set call to a JSNativeWrapper. To define a
 * self-hosted function, set selfHostedName to the name of a function
 * compiled during JSRuntime::initSelfHosting.
 */
#[repr(C)]
#[derive(Debug, Copy)]
pub struct JSFunctionSpec {
    pub name: *const ::std::os::raw::c_char,
    pub call: JSNativeWrapper,
    pub nargs: u16,
    pub flags: u16,
    pub selfHostedName: *const ::std::os::raw::c_char,
}
impl ::std::clone::Clone for JSFunctionSpec {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_JSFunctionSpec() {
    assert_eq!(::std::mem::size_of::<JSFunctionSpec>() , 40usize);
    assert_eq!(::std::mem::align_of::<JSFunctionSpec>() , 8usize);
}
#[repr(i8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum RootKind {
    BaseShape = 0,
    JitCode = 1,
    LazyScript = 2,
    Object = 3,
    ObjectGroup = 4,
    Script = 5,
    Shape = 6,
    String = 7,
    Symbol = 8,
    Id = 9,
    Value = 10,
    Traceable = 11,
    Limit = 12,
}
#[repr(C)]
pub struct ReadOnlyCompileOptions {
    pub _bindgen_opaque_blob: [u64; 11usize],
}
#[test]
fn bindgen_test_layout_ReadOnlyCompileOptions() {
    assert_eq!(::std::mem::size_of::<ReadOnlyCompileOptions>() , 88usize);
    assert_eq!(::std::mem::align_of::<ReadOnlyCompileOptions>() , 8usize);
}
pub type MutableHandleValue = MutableHandle<Value>;
pub type HandleValue = Handle<Value>;
pub type HandleObject = Handle<*mut JSObject>;
pub type AutoObjectVector = AutoVectorRooter<*mut JSObject>;
//pub static JSID_VOID: jsid = 0;
pub fn JS_SetNativeStackQuota(cx: *mut JSRuntime,
                              systemCodeStackSize: usize,
                              trustedScriptStackSize: usize,
                              untrustedScriptStackSize: usize){
    unimplemented!();
}

pub const JSCLASS_GLOBAL_APPLICATION_SLOTS: ::std::os::raw::c_uint = 5;
pub const JSCLASS_RESERVED_SLOTS_SHIFT: ::std::os::raw::c_uint = 8;
pub const JSCLASS_RESERVED_SLOTS_WIDTH: ::std::os::raw::c_uint = 8;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum JSProtoKey {
    JSProto_Null = 0,
    JSProto_Object = 1,
    JSProto_Function = 2,
    JSProto_Array = 3,
    JSProto_Boolean = 4,
    JSProto_JSON = 5,
    JSProto_Date = 6,
    JSProto_Math = 7,
    JSProto_Number = 8,
    JSProto_String = 9,
    JSProto_RegExp = 10,
    JSProto_Error = 11,
    JSProto_InternalError = 12,
    JSProto_EvalError = 13,
    JSProto_RangeError = 14,
    JSProto_ReferenceError = 15,
    JSProto_SyntaxError = 16,
    JSProto_TypeError = 17,
    JSProto_URIError = 18,
    JSProto_DebuggeeWouldRun = 19,
    JSProto_Iterator = 20,
    JSProto_StopIteration = 21,
    JSProto_ArrayBuffer = 22,
    JSProto_Int8Array = 23,
    JSProto_Uint8Array = 24,
    JSProto_Int16Array = 25,
    JSProto_Uint16Array = 26,
    JSProto_Int32Array = 27,
    JSProto_Uint32Array = 28,
    JSProto_Float32Array = 29,
    JSProto_Float64Array = 30,
    JSProto_Uint8ClampedArray = 31,
    JSProto_Proxy = 32,
    JSProto_WeakMap = 33,
    JSProto_Map = 34,
    JSProto_Set = 35,
    JSProto_DataView = 36,
    JSProto_Symbol = 37,
    JSProto_SharedArrayBuffer = 38,
    JSProto_Intl = 39,
    JSProto_TypedObject = 40,
    JSProto_Reflect = 41,
    JSProto_SIMD = 42,
    JSProto_WeakSet = 43,
    JSProto_TypedArray = 44,
    JSProto_Atomics = 45,
    JSProto_SavedFrame = 46,
    JSProto_Wasm = 47,
    JSProto_Promise = 48,
    JSProto_LIMIT = 49,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TraceKind {
    Object = 0,
    String = 1,
    Symbol = 2,
    Script = 3,
    Shape = 4,
    ObjectGroup = 5,
    Null = 6,
    BaseShape = 15,
    JitCode = 31,
    LazyScript = 47,
}
/**
 * This struct contains metadata passed from the DOM to the JS Engine for JIT
 * optimizations on DOM property accessors. Eventually, this should be made
 * available to general JSAPI users, but we are not currently ready to do so.
 */
#[repr(C)]
#[derive(Debug, Copy)]
pub struct JSJitInfo {
    pub call: *const ::std::os::raw::c_void,
    pub protoID: u16,
    pub depth: u16,
    /** The OpType that says what sort of function we are. */
    pub _bitfield_1: u32,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum JSJitInfo_OpType {
    Getter = 0,
    Setter = 1,
    Method = 2,
    StaticMethod = 3,
    InlinableNative = 4,
    OpTypeCount = 5,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum JSJitInfo_ArgType {
    String = 1,
    Integer = 2,
    Double = 4,
    Boolean = 8,
    Object = 16,
    Null = 32,
    Numeric = 6,
    Primitive = 47,
    ObjectOrNull = 48,
    Any = 63,
    ArgTypeListEnd = -2147483648,
}
/**
     * An enum that describes what this getter/setter/method aliases.  This
     * determines what things can be hoisted past this call, and if this
     * call is movable what it can be hoisted past.
     */
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum JSJitInfo_AliasSet {
    AliasNone = 0,
    AliasDOMSets = 1,
    AliasEverything = 2,
    AliasSetCount = 3,
}
impl JSJitInfo {
    #[inline]
    pub fn type_(&self) -> u32 {
        (self._bitfield_1 & (15usize as u32)) >> 0usize
    }
    #[inline]
    pub fn set_type_(&mut self, val: u8) {
        self._bitfield_1 &= !(15usize as u32);
        self._bitfield_1 |= ((val as u32) << 0usize) & (15usize as u32);
    }
    #[inline]
    pub fn aliasSet_(&self) -> u32 {
        (self._bitfield_1 & (240usize as u32)) >> 4usize
    }
    #[inline]
    pub fn set_aliasSet_(&mut self, val: u8) {
        self._bitfield_1 &= !(240usize as u32);
        self._bitfield_1 |= ((val as u32) << 4usize) & (240usize as u32);
    }
    #[inline]
    pub fn returnType_(&self) -> u32 {
        (self._bitfield_1 & (65280usize as u32)) >> 8usize
    }
    #[inline]
    pub fn set_returnType_(&mut self, val: u8) {
        self._bitfield_1 &= !(65280usize as u32);
        self._bitfield_1 |= ((val as u32) << 8usize) & (65280usize as u32);
    }
    #[inline]
    pub fn isInfallible(&self) -> u32 {
        (self._bitfield_1 & (65536usize as u32)) >> 16usize
    }
    #[inline]
    pub fn set_isInfallible(&mut self, val: bool) {
        self._bitfield_1 &= !(65536usize as u32);
        self._bitfield_1 |= ((val as u32) << 16usize) & (65536usize as u32);
    }
    #[inline]
    pub fn isMovable(&self) -> u32 {
        (self._bitfield_1 & (131072usize as u32)) >> 17usize
    }
    #[inline]
    pub fn set_isMovable(&mut self, val: bool) {
        self._bitfield_1 &= !(131072usize as u32);
        self._bitfield_1 |= ((val as u32) << 17usize) & (131072usize as u32);
    }
    #[inline]
    pub fn isEliminatable(&self) -> u32 {
        (self._bitfield_1 & (262144usize as u32)) >> 18usize
    }
    #[inline]
    pub fn set_isEliminatable(&mut self, val: bool) {
        self._bitfield_1 &= !(262144usize as u32);
        self._bitfield_1 |= ((val as u32) << 18usize) & (262144usize as u32);
    }
    #[inline]
    pub fn isAlwaysInSlot(&self) -> u32 {
        (self._bitfield_1 & (524288usize as u32)) >> 19usize
    }
    #[inline]
    pub fn set_isAlwaysInSlot(&mut self, val: bool) {
        self._bitfield_1 &= !(524288usize as u32);
        self._bitfield_1 |= ((val as u32) << 19usize) & (524288usize as u32);
    }
    #[inline]
    pub fn isLazilyCachedInSlot(&self) -> u32 {
        (self._bitfield_1 & (1048576usize as u32)) >> 20usize
    }
    #[inline]
    pub fn set_isLazilyCachedInSlot(&mut self, val: bool) {
        self._bitfield_1 &= !(1048576usize as u32);
        self._bitfield_1 |= ((val as u32) << 20usize) & (1048576usize as u32);
    }
    #[inline]
    pub fn isTypedMethod(&self) -> u32 {
        (self._bitfield_1 & (2097152usize as u32)) >> 21usize
    }
    #[inline]
    pub fn set_isTypedMethod(&mut self, val: bool) {
        self._bitfield_1 &= !(2097152usize as u32);
        self._bitfield_1 |= ((val as u32) << 21usize) & (2097152usize as u32);
    }
    #[inline]
    pub fn slotIndex(&self) -> u32 {
        (self._bitfield_1 & (4290772992usize as u32)) >> 22usize
    }
    #[inline]
    pub fn set_slotIndex(&mut self, val: u16) {
        self._bitfield_1 &= !(4290772992usize as u32);
        self._bitfield_1 |=
            ((val as u32) << 22usize) & (4290772992usize as u32);
    }
    pub const fn new_bitfield_1(type_: u8, aliasSet_: u8, returnType_: u8,
                                isInfallible: bool, isMovable: bool,
                                isEliminatable: bool, isAlwaysInSlot: bool,
                                isLazilyCachedInSlot: bool,
                                isTypedMethod: bool, slotIndex: u16) -> u32 {
        0 | ((type_ as u32) << 0u32) | ((aliasSet_ as u32) << 4u32) |
            ((returnType_ as u32) << 8u32) | ((isInfallible as u32) << 16u32)
            | ((isMovable as u32) << 17u32) |
            ((isEliminatable as u32) << 18u32) |
            ((isAlwaysInSlot as u32) << 19u32) |
            ((isLazilyCachedInSlot as u32) << 20u32) |
            ((isTypedMethod as u32) << 21u32) | ((slotIndex as u32) << 22u32)
    }
}
impl ::std::clone::Clone for JSJitInfo {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_JSJitInfo() {
    assert_eq!(::std::mem::size_of::<JSJitInfo>() , 16usize);
    assert_eq!(::std::mem::align_of::<JSJitInfo>() , 8usize);
}
pub type HandleId = Handle<jsid>;
/** These are the measurements used by Servo. */
#[repr(C)]
#[derive(Debug, Copy)]
pub struct ServoSizes {
    pub gcHeapUsed: usize,
    pub gcHeapUnused: usize,
    pub gcHeapAdmin: usize,
    pub gcHeapDecommitted: usize,
    pub mallocHeap: usize,
    pub nonHeap: usize,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ServoSizes_Kind {
    GCHeapUsed = 0,
    GCHeapUnused = 1,
    GCHeapAdmin = 2,
    GCHeapDecommitted = 3,
    MallocHeap = 4,
    NonHeap = 5,
    Ignore = 6,
}
impl ::std::clone::Clone for ServoSizes {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_ServoSizes() {
    assert_eq!(::std::mem::size_of::<ServoSizes>() , 48usize);
    assert_eq!(::std::mem::align_of::<ServoSizes>() , 8usize);
}
/**
 * Convenience class for imitating a JS level for-of loop. Typical usage:
 *
 *     ForOfIterator it(cx);
 *     if (!it.init(iterable))
 *       return false;
 *     RootedValue val(cx);
 *     while (true) {
 *       bool done;
 *       if (!it.next(&val, &done))
 *         return false;
 *       if (done)
 *         break;
 *       if (!DoStuff(cx, val))
 *         return false;
 *     }
 */
#[repr(C)]
#[unsafe_no_drop_flag]
#[derive(Debug)]
pub struct ForOfIterator {
    pub cx_: *mut JSContext,
    pub iterator: RootedObject,
    pub index: u32,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ForOfIterator_NonIterableBehavior {
    ThrowOnNonIterable = 0,
    AllowNonIterable = 1,
}
#[test]
fn bindgen_test_layout_ForOfIterator() {
    assert_eq!(::std::mem::size_of::<ForOfIterator>() , 40usize);
    assert_eq!(::std::mem::align_of::<ForOfIterator>() , 8usize);
}
impl ForOfIterator {
    /**
     * Initialize the iterator.  If AllowNonIterable is passed then if getting
     * the @@iterator property from iterable returns undefined init() will just
     * return true instead of throwing.  Callers must then check
     * valueIsIterable() before continuing with the iteration.
     */
    #[inline]
    pub unsafe fn init(&mut self, iterable: HandleValue,
                       nonIterableBehavior: ForOfIterator_NonIterableBehavior)
     -> bool {
        unimplemented!();
    }
    /**
     * Get the next value from the iterator.  If false *done is true
     * after this call, do not examine val.
     */
    #[inline]
    pub unsafe fn next(&mut self, val: MutableHandleValue, done: *mut bool)
     -> bool {
        unimplemented!();
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum JSValueType {
    JSVAL_TYPE_DOUBLE = 0,
    JSVAL_TYPE_INT32 = 1,
    JSVAL_TYPE_UNDEFINED = 2,
    JSVAL_TYPE_BOOLEAN = 3,
    JSVAL_TYPE_MAGIC = 4,
    JSVAL_TYPE_STRING = 5,
    JSVAL_TYPE_SYMBOL = 6,
    JSVAL_TYPE_PRIVATE_GCTHING = 7,
    JSVAL_TYPE_NULL = 8,
    JSVAL_TYPE_OBJECT = 12,
    JSVAL_TYPE_UNKNOWN = 32,
    JSVAL_TYPE_MISSING = 33,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct PropertyDescriptor {
    pub obj: *mut JSObject,
    pub attrs: ::std::os::raw::c_uint,
    pub getter: JSGetterOp,
    pub setter: JSSetterOp,
    pub value: Value,
}
impl ::std::clone::Clone for PropertyDescriptor {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_PropertyDescriptor() {
    assert_eq!(::std::mem::size_of::<PropertyDescriptor>() , 40usize);
    assert_eq!(::std::mem::align_of::<PropertyDescriptor>() , 8usize);
}
impl PropertyDescriptor {
    #[inline]
    pub unsafe fn trace(&mut self, trc: *mut JSTracer) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct JSClass {
    pub name: *const ::std::os::raw::c_char,
    pub flags: u32,
    pub cOps: *const JSClassOps,
    pub reserved: [*mut ::std::os::raw::c_void; 3usize],
}
impl ::std::clone::Clone for JSClass {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_JSClass() {
    assert_eq!(::std::mem::size_of::<JSClass>() , 48usize);
    assert_eq!(::std::mem::align_of::<JSClass>() , 8usize);
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct JSTracer {
    pub runtime_: *mut JSRuntime,
    //pub weakMapAction_: WeakMapTraceKind,
    pub tag_: JSTracer_TracerKindTag,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum JSTracer_TracerKindTag {
    Marking = 0,
    WeakMarking = 1,
    Tenuring = 2,
    Callback = 3,
}
impl ::std::clone::Clone for JSTracer {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_JSTracer() {
    assert_eq!(::std::mem::size_of::<JSTracer>() , 16usize);
    assert_eq!(::std::mem::align_of::<JSTracer>() , 8usize);
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Class {
    pub name: *const ::std::os::raw::c_char,
    pub flags: u32,
    pub cOps: *const ClassOps,
    pub spec: *const ClassSpec,
    pub ext: *const ClassExtension,
    pub oOps: *const ObjectOps,
}
impl ::std::clone::Clone for Class {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Class() {
    assert_eq!(::std::mem::size_of::<Class>() , 48usize);
    assert_eq!(::std::mem::align_of::<Class>() , 8usize);
}
pub type JSNative =
    ::std::option::Option<unsafe extern "C" fn(cx: *mut JSContext,
                                               argc: ::std::os::raw::c_uint,
                                               vp: *mut Value) -> bool>;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct IncludeUsedRval;
impl ::std::clone::Clone for IncludeUsedRval {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct NoUsedRval;
impl ::std::clone::Clone for NoUsedRval {
    fn clone(&self) -> Self { *self }
}
/**
 * CompartmentCreationOptions specifies options relevant to creating a new
 * compartment, that are either immutable characteristics of that compartment
 * or that are discarded after the compartment has been created.
 *
 * Access to these options on an existing compartment is read-only: if you
 * need particular selections, make them before you create the compartment.
 */
#[repr(C)]
#[derive(Debug, Copy)]
pub struct CompartmentCreationOptions {
  //  pub addonId_: *mut JSAddonId,
  //  pub traceGlobal_: JSTraceOp,
    pub zone_: CompartmentCreationOptions_jsapi_h_unnamed_8,
    pub invisibleToDebugger_: bool,
    pub mergeable_: bool,
    pub preserveJitCode_: bool,
    pub cloneSingletons_: bool,
    pub experimentalDateTimeFormatFormatToPartsEnabled_: bool,
    pub sharedMemoryAndAtomics_: bool,
    pub secureContext_: bool,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct CompartmentCreationOptions_jsapi_h_unnamed_8 {
    //pub spec: __BindgenUnionField<ZoneSpecifier>,
    //pub pointer: __BindgenUnionField<*mut ::std::os::raw::c_void>,
    //pub _bindgen_data_: u64,
}
/*
impl CompartmentCreationOptions_jsapi_h_unnamed_8 {
    pub unsafe fn spec(&mut self) -> *mut ZoneSpecifier {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn pointer(&mut self) -> *mut *mut ::std::os::raw::c_void {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
*/
impl ::std::clone::Clone for CompartmentCreationOptions_jsapi_h_unnamed_8 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_CompartmentCreationOptions_jsapi_h_unnamed_8() {
    assert_eq!(::std::mem::size_of::<CompartmentCreationOptions_jsapi_h_unnamed_8>()
               , 8usize);
    assert_eq!(::std::mem::align_of::<CompartmentCreationOptions_jsapi_h_unnamed_8>()
               , 8usize);
}
impl ::std::clone::Clone for CompartmentCreationOptions {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_CompartmentCreationOptions() {
    assert_eq!(::std::mem::size_of::<CompartmentCreationOptions>() , 32usize);
    assert_eq!(::std::mem::align_of::<CompartmentCreationOptions>() , 8usize);
}
impl CompartmentCreationOptions {
    #[inline]
    pub unsafe fn setZone(&mut self, spec: ZoneSpecifier)
     -> *mut CompartmentCreationOptions {
        unimplemented!();
    }
    #[inline]
    pub unsafe fn setSameZoneAs(&mut self, obj: *mut JSObject)
     -> *mut CompartmentCreationOptions {
        unimplemented!();
    }
    #[inline]
    pub unsafe fn getSharedMemoryAndAtomicsEnabled(&mut self) -> bool {
        unimplemented!();
    }
    #[inline]
    pub unsafe fn setSharedMemoryAndAtomicsEnabled(&mut self, flag: bool)
     -> *mut CompartmentCreationOptions {
        unimplemented!();
    }
}
/**
 * CompartmentBehaviors specifies behaviors of a compartment that can be
 * changed after the compartment's been created.
 */
#[repr(C)]
#[derive(Debug, Copy)]
pub struct CompartmentBehaviors {
   // pub version_: JSVersion,
    pub discardSource_: bool,
    pub disableLazyParsing_: bool,
    pub extraWarningsOverride_: CompartmentBehaviors_Override,
    pub singletonsAsTemplates_: bool,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct CompartmentBehaviors_Override {
    pub mode_: CompartmentBehaviors_Override_Mode,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CompartmentBehaviors_Override_Mode {
    Default = 0,
    ForceTrue = 1,
    ForceFalse = 2,
}
impl ::std::clone::Clone for CompartmentBehaviors_Override {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_CompartmentBehaviors_Override() {
    assert_eq!(::std::mem::size_of::<CompartmentBehaviors_Override>() ,
               4usize);
    assert_eq!(::std::mem::align_of::<CompartmentBehaviors_Override>() ,
               4usize);
}
impl ::std::clone::Clone for CompartmentBehaviors {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_CompartmentBehaviors() {
    assert_eq!(::std::mem::size_of::<CompartmentBehaviors>() , 16usize);
    assert_eq!(::std::mem::align_of::<CompartmentBehaviors>() , 4usize);
}
impl CompartmentBehaviors {
    #[inline]
    pub unsafe fn extraWarnings(&mut self, rt: *mut JSRuntime) -> bool {
        unimplemented!();
    }
}
#[repr(C)]
#[unsafe_no_drop_flag]
#[derive(Debug)]
pub struct AutoVectorRooterBase<T> {
   // pub _base: AutoGCRooter,
    pub _phantom0: ::std::marker::PhantomData<T>,
}
#[repr(C)]
#[unsafe_no_drop_flag]
#[derive(Debug)]
pub struct AutoVectorRooter<T> {
    pub _base: AutoVectorRooterBase<T>,
}
/**
 * Get a property named by id in obj.  Note the jsid id type -- id may
 * be a string (Unicode property identifier) or an int (element index).  The
 * *vp out parameter, on success, is the new property value after the action.
 */
pub type JSGetterOp =
    ::std::option::Option<unsafe extern "C" fn(cx: *mut JSContext,
                                               obj: HandleObject,
                                               id: HandleId,
                                               vp: MutableHandleValue)
                              -> bool>;
/** Add a property named by id to obj. */
pub type JSAddPropertyOp =
    ::std::option::Option<unsafe extern "C" fn(cx: *mut JSContext,
                                               obj: HandleObject,
                                               id: HandleId, v: HandleValue)
                              -> bool>;
/**
 * Set a property named by id in obj, treating the assignment as strict
 * mode code if strict is true. Note the jsid id type -- id may be a string
 * (Unicode property identifier) or an int (element index). The *vp out
 * parameter, on success, is the new property value after the
 * set.
 */
pub type JSSetterOp =
    ::std::option::Option<unsafe extern "C" fn(cx: *mut JSContext,
                                               obj: HandleObject,
                                               id: HandleId,
                                               vp: MutableHandleValue,
                                               result: *mut ObjectOpResult)
                              -> bool>;
/**
 * Per ES6, the [[DefineOwnProperty]] internal method has three different
 * possible outcomes:
 *
 * -   It can throw an exception (which we indicate by returning false).
 *
 * -   It can return true, indicating unvarnished success.
 *
 * -   It can return false, indicating "strict failure". The property could
 *     not be defined. It's an error, but no exception was thrown.
 *
 * It's not just [[DefineOwnProperty]]: all the mutating internal methods have
 * the same three outcomes. (The other affected internal methods are [[Set]],
 * [[Delete]], [[SetPrototypeOf]], and [[PreventExtensions]].)
 *
 * If you think this design is awful, you're not alone.  But as it's the
 * standard, we must represent these boolean "success" values somehow.
 * ObjectOpSuccess is the class for this. It's like a bool, but when it's false
 * it also stores an error code.
 *
 * Typical usage:
 *
 *     ObjectOpResult result;
 *     if (!DefineProperty(cx, obj, id, ..., result))
 *         return false;
 *     if (!result)
 *         return result.reportError(cx, obj, id);
 *
 * Users don't have to call `result.report()`; another possible ending is:
 *
 *     argv.rval().setBoolean(bool(result));
 *     return true;
 */
#[repr(C)]
#[derive(Debug, Copy)]
pub struct ObjectOpResult {
    /**
     * code_ is either one of the special codes OkCode or Uninitialized, or
     * an error code. For now the error codes are private to the JS engine;
     * they're defined in js/src/js.msg.
     *
     * code_ is uintptr_t (rather than uint32_t) for the convenience of the
     * JITs, which would otherwise have to deal with either padding or stack
     * alignment on 64-bit platforms.
     */
    pub code_: usize,
}
#[repr(i64)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ObjectOpResult_SpecialCodes { OkCode = 0, Uninitialized = -1, }
impl ::std::clone::Clone for ObjectOpResult {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_ObjectOpResult() {
    assert_eq!(::std::mem::size_of::<ObjectOpResult>() , 8usize);
    assert_eq!(::std::mem::align_of::<ObjectOpResult>() , 8usize);
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ZoneSpecifier { FreshZone = 0, SystemZone = 1, }
#[repr(C)]
#[derive(Debug, Copy)]
pub struct JSClassOps {
    pub addProperty: JSAddPropertyOp,
    pub delProperty: JSDeletePropertyOp,
    pub getProperty: JSGetterOp,
    pub setProperty: JSSetterOp,
    pub enumerate: JSEnumerateOp,
    pub resolve: JSResolveOp,
    pub mayResolve: JSMayResolveOp,
    pub finalize: JSFinalizeOp,
    pub call: JSNative,
    pub hasInstance: JSHasInstanceOp,
    pub construct: JSNative,
    pub trace: JSTraceOp,
}
impl ::std::clone::Clone for JSClassOps {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_JSClassOps() {
    assert_eq!(::std::mem::size_of::<JSClassOps>() , 96usize);
    assert_eq!(::std::mem::align_of::<JSClassOps>() , 8usize);
}
pub type GetElementsOp =
    ::std::option::Option<unsafe extern "C" fn(cx: *mut JSContext,
                                               obj: HandleObject, begin: u32,
                                               end: u32,
                                               adder: *mut ElementAdder)
                              -> bool>;
pub type FinalizeOp =
    ::std::option::Option<unsafe extern "C" fn(fop: *mut FreeOp,
                                               obj: *mut JSObject)>;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct ClassOps {
    pub addProperty: JSAddPropertyOp,
    pub delProperty: JSDeletePropertyOp,
    pub getProperty: JSGetterOp,
    pub setProperty: JSSetterOp,
    pub enumerate: JSEnumerateOp,
    pub resolve: JSResolveOp,
    pub mayResolve: JSMayResolveOp,
    pub finalize: FinalizeOp,
    pub call: JSNative,
    pub hasInstance: JSHasInstanceOp,
    pub construct: JSNative,
    pub trace: JSTraceOp,
}
impl ::std::clone::Clone for ClassOps {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_ClassOps() {
    assert_eq!(::std::mem::size_of::<ClassOps>() , 96usize);
    assert_eq!(::std::mem::align_of::<ClassOps>() , 8usize);
}
/** Callback for the creation of constructor and prototype objects. */
pub type ClassObjectCreationOp =
    ::std::option::Option<unsafe extern "C" fn(cx: *mut JSContext,
                                               key: JSProtoKey)
                              -> *mut JSObject>;
/** Callback for custom post-processing after class initialization via ClassSpec. */
pub type FinishClassInitOp =
    ::std::option::Option<unsafe extern "C" fn(cx: *mut JSContext,
                                               ctor: HandleObject,
                                               proto: HandleObject) -> bool>;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct ClassSpec {
    pub createConstructor_: ClassObjectCreationOp,
    pub createPrototype_: ClassObjectCreationOp,
    pub constructorFunctions_: *const JSFunctionSpec,
    pub constructorProperties_: *const JSPropertySpec,
    pub prototypeFunctions_: *const JSFunctionSpec,
    pub prototypeProperties_: *const JSPropertySpec,
    pub finishInit_: FinishClassInitOp,
    pub flags: usize,
}
impl ::std::clone::Clone for ClassSpec {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_ClassSpec() {
    assert_eq!(::std::mem::size_of::<ClassSpec>() , 64usize);
    assert_eq!(::std::mem::align_of::<ClassSpec>() , 8usize);
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct ClassExtension {
    /**
     * If an object is used as a key in a weakmap, it may be desirable for the
     * garbage collector to keep that object around longer than it otherwise
     * would. A common case is when the key is a wrapper around an object in
     * another compartment, and we want to avoid collecting the wrapper (and
     * removing the weakmap entry) as long as the wrapped object is alive. In
     * that case, the wrapped object is returned by the wrapper's
     * weakmapKeyDelegateOp hook. As long as the wrapper is used as a weakmap
     * key, it will not be collected (and remain in the weakmap) until the
     * wrapped object is collected.
     */
    pub weakmapKeyDelegateOp: JSWeakmapKeyDelegateOp,
    /**
     * Optional hook called when an object is moved by a compacting GC.
     *
     * There may exist weak pointers to an object that are not traced through
     * when the normal trace APIs are used, for example objects in the wrapper
     * cache. This hook allows these pointers to be updated.
     *
     * Note that this hook can be called before JS_NewObject() returns if a GC
     * is triggered during construction of the object. This can happen for
     * global objects for example.
     */
    pub objectMovedOp: JSObjectMovedOp,
}
impl ::std::clone::Clone for ClassExtension {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_ClassExtension() {
    assert_eq!(::std::mem::size_of::<ClassExtension>() , 16usize);
    assert_eq!(::std::mem::align_of::<ClassExtension>() , 8usize);
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct ObjectOps {
    pub lookupProperty: LookupPropertyOp,
    pub defineProperty: DefinePropertyOp,
    pub hasProperty: HasPropertyOp,
    pub getProperty: GetPropertyOp,
    pub setProperty: SetPropertyOp,
    pub getOwnPropertyDescriptor: GetOwnPropertyOp,
    pub deleteProperty: DeletePropertyOp,
    pub watch: WatchOp,
    pub unwatch: UnwatchOp,
    pub getElements: GetElementsOp,
    pub enumerate: JSNewEnumerateOp,
    pub funToString: JSFunToStringOp,
}
impl ::std::clone::Clone for ObjectOps {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_ObjectOps() {
    assert_eq!(::std::mem::size_of::<ObjectOps>() , 96usize);
    assert_eq!(::std::mem::align_of::<ObjectOps>() , 8usize);
}
/**
 * Delete a property named by id in obj.
 *
 * If an error occurred, return false as per normal JSAPI error practice.
 *
 * If no error occurred, but the deletion attempt wasn't allowed (perhaps
 * because the property was non-configurable), call result.fail() and
 * return true.  This will cause |delete obj[id]| to evaluate to false in
 * non-strict mode code, and to throw a TypeError in strict mode code.
 *
 * If no error occurred and the deletion wasn't disallowed (this is *not* the
 * same as saying that a deletion actually occurred -- deleting a non-existent
 * property, or an inherited property, is allowed -- it's just pointless),
 * call result.succeed() and return true.
 */
pub type JSDeletePropertyOp =
    ::std::option::Option<unsafe extern "C" fn(cx: *mut JSContext,
                                               obj: HandleObject,
                                               id: HandleId,
                                               result: *mut ObjectOpResult)
                              -> bool>;
/**
 * The type of ObjectOps::enumerate. This callback overrides a portion of
 * SpiderMonkey's default [[Enumerate]] internal method. When an ordinary object
 * is enumerated, that object and each object on its prototype chain is tested
 * for an enumerate op, and those ops are called in order. The properties each
 * op adds to the 'properties' vector are added to the set of values the for-in
 * loop will iterate over. All of this is nonstandard.
 *
 * An object is "enumerated" when it's the target of a for-in loop or
 * JS_Enumerate(). The callback's job is to populate 'properties' with the
 * object's property keys. If `enumerableOnly` is true, the callback should only
 * add enumerable properties.
 */
pub type JSNewEnumerateOp =
    ::std::option::Option<unsafe extern "C" fn(cx: *mut JSContext,
                                               obj: HandleObject,
                                               properties: *mut AutoIdVector,
                                               enumerableOnly: bool) -> bool>;
/**
 * The old-style JSClass.enumerate op should define all lazy properties not
 * yet reflected in obj.
 */
pub type JSEnumerateOp =
    ::std::option::Option<unsafe extern "C" fn(cx: *mut JSContext,
                                               obj: HandleObject) -> bool>;
/**
 * The type of ObjectOps::funToString.  This callback allows an object to
 * provide a custom string to use when Function.prototype.toString is invoked on
 * that object.  A null return value means OOM.
 */
pub type JSFunToStringOp =
    ::std::option::Option<unsafe extern "C" fn(cx: *mut JSContext,
                                               obj: HandleObject,
                                               indent: ::std::os::raw::c_uint)
                              -> *mut JSString>;
/**
 * Resolve a lazy property named by id in obj by defining it directly in obj.
 * Lazy properties are those reflected from some peer native property space
 * (e.g., the DOM attributes for a given node reflected as obj) on demand.
 *
 * JS looks for a property in an object, and if not found, tries to resolve
 * the given id. *resolvedp should be set to true iff the property was defined
 * on |obj|.
 */
pub type JSResolveOp =
    ::std::option::Option<unsafe extern "C" fn(cx: *mut JSContext,
                                               obj: HandleObject,
                                               id: HandleId,
                                               resolvedp: *mut bool) -> bool>;
/**
 * A class with a resolve hook can optionally have a mayResolve hook. This hook
 * must have no side effects and must return true for a given id if the resolve
 * hook may resolve this id. This is useful when we're doing a "pure" lookup: if
 * mayResolve returns false, we know we don't have to call the effectful resolve
 * hook.
 *
 * maybeObj, if non-null, is the object on which we're doing the lookup. This
 * can be nullptr: during JIT compilation we sometimes know the Class but not
 * the object.
 */
pub type JSMayResolveOp =
    ::std::option::Option<unsafe extern "C" fn(names: *const JSAtomState,
                                               id: jsid,
                                               maybeObj: *mut JSObject)
                              -> bool>;
/**
 * Finalize obj, which the garbage collector has determined to be unreachable
 * from other live objects or from GC roots.  Obviously, finalizers must never
 * store a reference to obj.
 */
pub type JSFinalizeOp =
    ::std::option::Option<unsafe extern "C" fn(fop: *mut JSFreeOp,
                                               obj: *mut JSObject)>;
/**
 * Check whether v is an instance of obj.  Return false on error or exception,
 * true on success with true in *bp if v is an instance of obj, false in
 * *bp otherwise.
 */
pub type JSHasInstanceOp =
    ::std::option::Option<unsafe extern "C" fn(cx: *mut JSContext,
                                               obj: HandleObject,
                                               vp: MutableHandleValue,
                                               bp: *mut bool) -> bool>;
/**
 * Function type for trace operation of the class called to enumerate all
 * traceable things reachable from obj's private data structure. For each such
 * thing, a trace implementation must call one of the JS_Call*Tracer variants
 * on the thing.
 *
 * JSTraceOp implementation can assume that no other threads mutates object
 * state. It must not change state of the object or corresponding native
 * structures. The only exception for this rule is the case when the embedding
 * needs a tight integration with GC. In that case the embedding can check if
 * the traversal is a part of the marking phase through calling
 * JS_IsGCMarkingTracer and apply a special code like emptying caches or
 * marking its native structures.
 */
pub type JSTraceOp =
    ::std::option::Option<unsafe extern "C" fn(trc: *mut JSTracer,
                                               obj: *mut JSObject)>;
pub enum FreeOp { }
pub type JSWeakmapKeyDelegateOp =
    ::std::option::Option<unsafe extern "C" fn(obj: *mut JSObject)
                              -> *mut JSObject>;
pub type JSObjectMovedOp =
    ::std::option::Option<unsafe extern "C" fn(obj: *mut JSObject,
                                               old: *const JSObject)>;
pub type LookupPropertyOp =
    ::std::option::Option<unsafe extern "C" fn(cx: *mut JSContext,
                                               obj: HandleObject,
                                               id: HandleId,
                                               objp: MutableHandleObject,
                                               propp:
                                                   MutableHandle<*mut Shape>)
                              -> bool>;
pub type DefinePropertyOp =
    ::std::option::Option<unsafe extern "C" fn(cx: *mut JSContext,
                                               obj: HandleObject,
                                               id: HandleId,
                                               desc:
                                                   Handle<PropertyDescriptor>,
                                               result: *mut ObjectOpResult)
                              -> bool>;
pub type HasPropertyOp =
    ::std::option::Option<unsafe extern "C" fn(cx: *mut JSContext,
                                               obj: HandleObject,
                                               id: HandleId,
                                               foundp: *mut bool) -> bool>;
pub type GetPropertyOp =
    ::std::option::Option<unsafe extern "C" fn(cx: *mut JSContext,
                                               obj: HandleObject,
                                               receiver: HandleValue,
                                               id: HandleId,
                                               vp: MutableHandleValue)
                              -> bool>;
pub type SetPropertyOp =
    ::std::option::Option<unsafe extern "C" fn(cx: *mut JSContext,
                                               obj: HandleObject,
                                               id: HandleId, v: HandleValue,
                                               receiver: HandleValue,
                                               result: *mut ObjectOpResult)
                              -> bool>;
pub type GetOwnPropertyOp =
    ::std::option::Option<unsafe extern "C" fn(cx: *mut JSContext,
                                               obj: HandleObject,
                                               id: HandleId,
                                               desc:
                                                   MutableHandle<PropertyDescriptor>)
                              -> bool>;
pub type DeletePropertyOp =
    ::std::option::Option<unsafe extern "C" fn(cx: *mut JSContext,
                                               obj: HandleObject,
                                               id: HandleId,
                                               result: *mut ObjectOpResult)
                              -> bool>;
pub type WatchOp =
    ::std::option::Option<unsafe extern "C" fn(cx: *mut JSContext,
                                               obj: HandleObject,
                                               id: HandleId,
                                               callable: HandleObject)
                              -> bool>;
pub type UnwatchOp =
    ::std::option::Option<unsafe extern "C" fn(cx: *mut JSContext,
                                               obj: HandleObject,
                                               id: HandleId) -> bool>;
#[repr(C)]
#[unsafe_no_drop_flag]
#[derive(Debug)]
pub struct ElementAdder {
    pub resObj_: RootedObject,
    pub vp_: *mut Value,
    pub index_: u32,
    pub getBehavior_: ElementAdder_GetBehavior,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ElementAdder_GetBehavior {
    CheckHasElemPreserveHoles = 0,
    GetElement = 1,
}
#[test]
fn bindgen_test_layout_ElementAdder() {
    assert_eq!(::std::mem::size_of::<ElementAdder>() , 40usize);
    assert_eq!(::std::mem::align_of::<ElementAdder>() , 8usize);
}
impl ElementAdder {
    #[inline]
    pub unsafe fn append(&mut self, cx: *mut JSContext, v: HandleValue)
     -> bool {
        unimplemented!();
    }
    #[inline]
    pub unsafe fn appendHole(&mut self) {
        unimplemented!();
    }
}
pub enum JSAtomState { }
#[repr(C)]
#[derive(Debug, Copy)]
pub struct BaseShape {
    pub clasp_: *const Class,
    pub parent: *mut JSObject,
}
impl ::std::clone::Clone for BaseShape {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_BaseShape() {
    assert_eq!(::std::mem::size_of::<BaseShape>() , 16usize);
    assert_eq!(::std::mem::align_of::<BaseShape>() , 8usize);
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Shape {
    pub base: *mut BaseShape,
    pub _1: jsid,
    pub slotInfo: u32,
}
impl ::std::clone::Clone for Shape {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Shape() {
    assert_eq!(::std::mem::size_of::<Shape>() , 24usize);
    assert_eq!(::std::mem::align_of::<Shape>() , 8usize);
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct JSFreeOp {
    pub runtime_: *mut JSRuntime,
}
impl ::std::clone::Clone for JSFreeOp {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_JSFreeOp() {
    assert_eq!(::std::mem::size_of::<JSFreeOp>() , 8usize);
    assert_eq!(::std::mem::align_of::<JSFreeOp>() , 8usize);
}
pub fn ObjectClassName(cx: *mut JSContext, obj: HandleObject)
 -> *const ::std::os::raw::c_char{
    unimplemented!();
 }    
 pub fn SetDOMProxyInformation(domProxyHandlerFamily:
                              *const ::std::os::raw::c_void,
                              domProxyExpandoSlot: u32,
                              domProxyShadowsCheck: DOMProxyShadowsCheck){
  unimplemented!();
}
pub fn JS_RequestInterruptCallback(rt: *mut JSRuntime){
    unimplemented!();
}
pub fn GetGlobalForObjectCrossCompartment(obj: *mut JSObject)
 -> *mut JSObject{
    unimplemented!();
 }
pub fn JS_GetPropertyDescriptorById(cx: *mut JSContext, obj: HandleObject,
                                    id: HandleId,
                                    desc:
                                        MutableHandle<PropertyDescriptor>)
 -> bool{
    unimplemented!();
}


pub fn IsCallable(obj: *mut JSObject) -> bool{
    unimplemented!();
}

pub fn JS_WrapObject(cx: *mut JSContext, objp: MutableHandleObject)
-> bool{
    unimplemented!();
}

pub fn JS_EnterCompartment(cx: *mut JSContext, target: *mut JSObject)
-> *mut JSCompartment{
    unimplemented!();
}
pub fn JS_LeaveCompartment(cx: *mut JSContext,
               oldCompartment: *mut JSCompartment){
    unimplemented!();
}
pub fn JS_GetArrayBufferViewType(obj: *mut JSObject) -> Type{
    unimplemented!();
}
/**
 * Scalar types that can appear in typed arrays and typed objects.  The enum
 * values must to be kept in sync with the JS_SCALARTYPEREPR_ constants, as
 * well as the TypedArrayObject::classes and TypedArrayObject::protoClasses
 * definitions.
 */
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Type {
    Int8 = 0,
    Uint8 = 1,
    Int16 = 2,
    Uint16 = 3,
    Int32 = 4,
    Uint32 = 5,
    Float32 = 6,
    Float64 = 7,
    Uint8Clamped = 8,
    MaxTypedArrayViewType = 9,
    Float32x4 = 10,
    Int8x16 = 11,
    Int16x8 = 12,
    Int32x4 = 13,
}
pub type DOMProxyShadowsCheck =
    ::std::option::Option<unsafe extern "C" fn(cx: *mut JSContext,
                                               object: HandleObject,
                                               id: HandleId)
                              -> DOMProxyShadowsResult>;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum DOMProxyShadowsResult {
    ShadowCheckFailed = 0,
    Shadows = 1,
    DoesntShadow = 2,
    DoesntShadowUnique = 3,
    ShadowsViaDirectExpando = 4,
    ShadowsViaIndirectExpando = 5,
}


pub fn JS_GetProperty(cx: *mut JSContext, obj: HandleObject,
                      name: *const ::std::os::raw::c_char,
                      vp: MutableHandleValue) -> bool{
    unimplemented!();
}
pub fn JS_GetLatin1StringCharsAndLength(cx: *mut JSContext,
                                        nogc: *const AutoCheckCannotGC,
                                        str: *mut JSString,
                                        length: *mut usize)
 -> *const Latin1Char{
    unimplemented!();
}
pub fn JS_GetObjectAsArrayBufferView(obj: *mut JSObject, length: *mut u32,
                                     isSharedMemory: *mut bool,
                                     data: *mut *mut u8) -> *mut JSObject{
    unimplemented!();
}
pub fn JS_GetReservedSlot(obj: *mut JSObject, index: u32) -> Value{
    unimplemented!();
}
pub fn JS_GetTwoByteStringCharsAndLength(cx: *mut JSContext,
                                         nogc: *const AutoCheckCannotGC,
                                         str: *mut JSString,
                                         length: *mut usize)
 -> *const ::std::os::raw::c_ushort{
    unimplemented!();
}
pub fn ToWindowProxyIfWindow(obj: *mut JSObject) -> *mut JSObject{
    unimplemented!();
}
/**
 * Place AutoCheckCannotGC in scopes that you believe can never GC. These
 * annotations will be verified both dynamically via AutoAssertOnGC, and
 * statically with the rooting hazard analysis (implemented by making the
 * analysis consider AutoCheckCannotGC to be a GC pointer, and therefore
 * complain if it is live across a GC call.) It is useful when dealing with
 * internal pointers to GC things where the GC thing itself may not be present
 * for the static analysis: e.g. acquiring inline chars from a JSString* on the
 * heap.
 */
#[repr(C)]
#[unsafe_no_drop_flag]
#[derive(Debug)]
pub struct AutoCheckCannotGC {
    pub _base: AutoAssertOnGC,
}
/**
 * Assert if a GC occurs while this class is live. This class does not disable
 * the static rooting hazard analysis.
 */
#[repr(C)]
#[unsafe_no_drop_flag]
#[derive(Debug)]
pub struct AutoAssertOnGC;
pub type Latin1Char = ::std::os::raw::c_uchar;
