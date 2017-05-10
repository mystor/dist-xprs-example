//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXPConnect.idl
//


#[repr(C)]
pub struct nsIXPConnectJSObjectHolder {
    vtable: *const nsIXPConnectJSObjectHolderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPConnectJSObjectHolder {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x73e6ff4a, 0xab99, 0x4d99,
            [0xac, 0x00, 0xba, 0x39, 0xcc, 0xb8, 0xe4, 0xd7])
    }
}

unsafe impl RefCounted for nsIXPConnectJSObjectHolder {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// Enable coercing to ourselves
pub trait nsIXPConnectJSObjectHolderCoerce {
    fn coerce_from(v: &nsIXPConnectJSObjectHolder) -> &Self;
}

impl nsIXPConnectJSObjectHolderCoerce for nsIXPConnectJSObjectHolder {
    #[inline]
    fn coerce_from(v: &nsIXPConnectJSObjectHolder) -> &Self {
        v
    }
}

impl nsIXPConnectJSObjectHolder {
    #[inline]
    pub fn coerce<T: nsIXPConnectJSObjectHolderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPConnectJSObjectHolder {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPConnectJSObjectHolderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPConnectJSObjectHolder) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPConnectJSObjectHolderVTable {
    pub __base: nsISupportsVTable,

    /* [nostdcall,notxpcom] JSObjectPtr GetJSObject (); */
    /// Unable to call function as its signature contains a non-rust type
    pub GetJSObject: *const ::libc::c_void,

}


impl nsIXPConnectJSObjectHolder {
    /* [nostdcall,notxpcom] JSObjectPtr GetJSObject (); */


}


#[repr(C)]
pub struct nsIXPConnectWrappedNative {
    vtable: *const nsIXPConnectWrappedNativeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPConnectWrappedNative {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe787be29, 0xdb5d, 0x4a45,
            [0xa3, 0xd6, 0x1d, 0xe1, 0xd6, 0xb8, 0x5c, 0x30])
    }
}

unsafe impl RefCounted for nsIXPConnectWrappedNative {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// Enable coercing to ourselves
pub trait nsIXPConnectWrappedNativeCoerce {
    fn coerce_from(v: &nsIXPConnectWrappedNative) -> &Self;
}

impl nsIXPConnectWrappedNativeCoerce for nsIXPConnectWrappedNative {
    #[inline]
    fn coerce_from(v: &nsIXPConnectWrappedNative) -> &Self {
        v
    }
}

impl nsIXPConnectWrappedNative {
    #[inline]
    pub fn coerce<T: nsIXPConnectWrappedNativeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPConnectWrappedNative {
    type Target = nsIXPConnectJSObjectHolder;
    #[inline]
    fn deref(&self) -> &nsIXPConnectJSObjectHolder {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIXPConnectJSObjectHolderCoerce> nsIXPConnectWrappedNativeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPConnectWrappedNative) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPConnectWrappedNativeVTable {
    pub __base: nsIXPConnectJSObjectHolderVTable,

    /* readonly attribute nsISupports Native; */
    pub get_Native: unsafe extern "C" fn (this: *const nsIXPConnectWrappedNative, aNative: *mut *const nsISupports) -> nsresult,

    /* readonly attribute JSObjectPtr JSObjectPrototype; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_JSObjectPrototype: *const ::libc::c_void,

    /* nsIInterfaceInfo FindInterfaceWithMember (in JSHandleId nameID); */
    /// Unable to call function as its signature contains a non-rust type
    pub FindInterfaceWithMember: *const ::libc::c_void,

    /* nsIInterfaceInfo FindInterfaceWithName (in JSHandleId nameID); */
    /// Unable to call function as its signature contains a non-rust type
    pub FindInterfaceWithName: *const ::libc::c_void,

    /* [notxpcom] bool HasNativeMember (in JSHandleId name); */
    /// Unable to call function as its signature contains a non-rust type
    pub HasNativeMember: *const ::libc::c_void,

    /* void debugDump (in short depth); */
    pub debugDump: unsafe extern "C" fn (this: *const nsIXPConnectWrappedNative, depth: libc::int16_t) -> nsresult,

}


impl nsIXPConnectWrappedNative {
    /* readonly attribute nsISupports Native; */
    #[inline]
    pub unsafe fn get_Native(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_Native)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute JSObjectPtr JSObjectPrototype; */


    /* nsIInterfaceInfo FindInterfaceWithMember (in JSHandleId nameID); */


    /* nsIInterfaceInfo FindInterfaceWithName (in JSHandleId nameID); */


    /* [notxpcom] bool HasNativeMember (in JSHandleId name); */


    /* void debugDump (in short depth); */
    #[inline]
    pub unsafe fn debugDump(&self, depth: libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).debugDump)(self as *const _, depth) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIXPConnectWrappedJS {
    vtable: *const nsIXPConnectWrappedJSVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPConnectWrappedJS {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3a01b0d6, 0x074b, 0x49ed,
            [0xba, 0xc3, 0x08, 0xc7, 0x63, 0x66, 0xca, 0xe4])
    }
}

unsafe impl RefCounted for nsIXPConnectWrappedJS {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// Enable coercing to ourselves
pub trait nsIXPConnectWrappedJSCoerce {
    fn coerce_from(v: &nsIXPConnectWrappedJS) -> &Self;
}

impl nsIXPConnectWrappedJSCoerce for nsIXPConnectWrappedJS {
    #[inline]
    fn coerce_from(v: &nsIXPConnectWrappedJS) -> &Self {
        v
    }
}

impl nsIXPConnectWrappedJS {
    #[inline]
    pub fn coerce<T: nsIXPConnectWrappedJSCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPConnectWrappedJS {
    type Target = nsIXPConnectJSObjectHolder;
    #[inline]
    fn deref(&self) -> &nsIXPConnectJSObjectHolder {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIXPConnectJSObjectHolderCoerce> nsIXPConnectWrappedJSCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPConnectWrappedJS) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPConnectWrappedJSVTable {
    pub __base: nsIXPConnectJSObjectHolderVTable,

    /* readonly attribute nsIInterfaceInfo InterfaceInfo; */
    pub get_InterfaceInfo: unsafe extern "C" fn (this: *const nsIXPConnectWrappedJS, aInterfaceInfo: *mut *const nsIInterfaceInfo) -> nsresult,

    /* readonly attribute nsIIDPtr InterfaceIID; */
    pub get_InterfaceIID: unsafe extern "C" fn (this: *const nsIXPConnectWrappedJS, aInterfaceIID: *mut *const nsIID) -> nsresult,

    /* void debugDump (in short depth); */
    pub debugDump: unsafe extern "C" fn (this: *const nsIXPConnectWrappedJS, depth: libc::int16_t) -> nsresult,

    /* void aggregatedQueryInterface (in nsIIDRef uuid, [iid_is (uuid), retval] out nsQIResult result); */
    pub aggregatedQueryInterface: unsafe extern "C" fn (this: *const nsIXPConnectWrappedJS, uuid: *const nsIID, result: *mut *const libc::c_void) -> nsresult,

}


impl nsIXPConnectWrappedJS {
    /* readonly attribute nsIInterfaceInfo InterfaceInfo; */
    #[inline]
    pub unsafe fn get_InterfaceInfo(&self, ) -> Result<Option<RefPtr<nsIInterfaceInfo>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_InterfaceInfo)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIIDPtr InterfaceIID; */
    #[inline]
    pub unsafe fn get_InterfaceIID(&self, ) -> Result<*const nsIID, nsresult> {
        let mut _retval: *const nsIID = ::std::ptr::null();
        match ((*self.vtable).get_InterfaceIID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void debugDump (in short depth); */
    #[inline]
    pub unsafe fn debugDump(&self, depth: libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).debugDump)(self as *const _, depth) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void aggregatedQueryInterface (in nsIIDRef uuid, [iid_is (uuid), retval] out nsQIResult result); */
    #[inline]
    pub unsafe fn aggregatedQueryInterface<T: XpCom>(&self, ) -> Result<Option<RefPtr<T>>, nsresult> {
        let mut result : GetterAddrefs<T> = GetterAddrefs::new();
        match ((*self.vtable).aggregatedQueryInterface)(self as *const _, &T::iid(), result.ptr() as *mut *const T as *mut *const ::libc::c_void) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(result.refptr())
    }

}


#[repr(C)]
pub struct nsIXPConnectWrappedJSUnmarkGray {
    vtable: *const nsIXPConnectWrappedJSUnmarkGrayVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPConnectWrappedJSUnmarkGray {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc02a0ce6, 0x275f, 0x4ea1,
            [0x9c, 0x23, 0x08, 0x49, 0x48, 0x98, 0xb0, 0x70])
    }
}

unsafe impl RefCounted for nsIXPConnectWrappedJSUnmarkGray {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// Enable coercing to ourselves
pub trait nsIXPConnectWrappedJSUnmarkGrayCoerce {
    fn coerce_from(v: &nsIXPConnectWrappedJSUnmarkGray) -> &Self;
}

impl nsIXPConnectWrappedJSUnmarkGrayCoerce for nsIXPConnectWrappedJSUnmarkGray {
    #[inline]
    fn coerce_from(v: &nsIXPConnectWrappedJSUnmarkGray) -> &Self {
        v
    }
}

impl nsIXPConnectWrappedJSUnmarkGray {
    #[inline]
    pub fn coerce<T: nsIXPConnectWrappedJSUnmarkGrayCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPConnectWrappedJSUnmarkGray {
    type Target = nsIXPConnectWrappedJS;
    #[inline]
    fn deref(&self) -> &nsIXPConnectWrappedJS {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIXPConnectWrappedJSCoerce> nsIXPConnectWrappedJSUnmarkGrayCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPConnectWrappedJSUnmarkGray) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPConnectWrappedJSUnmarkGrayVTable {
    pub __base: nsIXPConnectWrappedJSVTable,

}


impl nsIXPConnectWrappedJSUnmarkGray {
}


#[repr(C)]
pub struct nsIXPCWrappedJSObjectGetter {
    vtable: *const nsIXPCWrappedJSObjectGetterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCWrappedJSObjectGetter {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x254bb2e0, 0x6439, 0x11d4,
            [0x8f, 0xe0, 0x00, 0x10, 0xa4, 0xe7, 0x3d, 0x9a])
    }
}

unsafe impl RefCounted for nsIXPCWrappedJSObjectGetter {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// Enable coercing to ourselves
pub trait nsIXPCWrappedJSObjectGetterCoerce {
    fn coerce_from(v: &nsIXPCWrappedJSObjectGetter) -> &Self;
}

impl nsIXPCWrappedJSObjectGetterCoerce for nsIXPCWrappedJSObjectGetter {
    #[inline]
    fn coerce_from(v: &nsIXPCWrappedJSObjectGetter) -> &Self {
        v
    }
}

impl nsIXPCWrappedJSObjectGetter {
    #[inline]
    pub fn coerce<T: nsIXPCWrappedJSObjectGetterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCWrappedJSObjectGetter {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPCWrappedJSObjectGetterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCWrappedJSObjectGetter) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCWrappedJSObjectGetterVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsISupports neverCalled; */
    pub get_neverCalled: unsafe extern "C" fn (this: *const nsIXPCWrappedJSObjectGetter, aNeverCalled: *mut *const nsISupports) -> nsresult,

}


impl nsIXPCWrappedJSObjectGetter {
    /* readonly attribute nsISupports neverCalled; */
    #[inline]
    pub unsafe fn get_neverCalled(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_neverCalled)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct nsIXPCFunctionThisTranslator {
    vtable: *const nsIXPCFunctionThisTranslatorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCFunctionThisTranslator {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf5f84b70, 0x92eb, 0x41f1,
            [0xa1, 0xdd, 0x2e, 0xaa, 0xc0, 0xed, 0x56, 0x4c])
    }
}

unsafe impl RefCounted for nsIXPCFunctionThisTranslator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// Enable coercing to ourselves
pub trait nsIXPCFunctionThisTranslatorCoerce {
    fn coerce_from(v: &nsIXPCFunctionThisTranslator) -> &Self;
}

impl nsIXPCFunctionThisTranslatorCoerce for nsIXPCFunctionThisTranslator {
    #[inline]
    fn coerce_from(v: &nsIXPCFunctionThisTranslator) -> &Self {
        v
    }
}

impl nsIXPCFunctionThisTranslator {
    #[inline]
    pub fn coerce<T: nsIXPCFunctionThisTranslatorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCFunctionThisTranslator {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPCFunctionThisTranslatorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCFunctionThisTranslator) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCFunctionThisTranslatorVTable {
    pub __base: nsISupportsVTable,

    /* nsISupports TranslateThis (in nsISupports aInitialThis); */
    pub TranslateThis: unsafe extern "C" fn (this: *const nsIXPCFunctionThisTranslator, aInitialThis: *const nsISupports, _retval: *mut *const nsISupports) -> nsresult,

}


impl nsIXPCFunctionThisTranslator {
    /* nsISupports TranslateThis (in nsISupports aInitialThis); */
    #[inline]
    pub unsafe fn TranslateThis(&self, aInitialThis: Option<&nsISupports>) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).TranslateThis)(self as *const _, aInitialThis.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


pub mod nsIXPConnect_consts {
    pub const INIT_JS_STANDARD_CLASSES: i64 = 1;
    pub const DONT_FIRE_ONNEWGLOBALHOOK: i64 = 2;
    pub const OMIT_COMPONENTS_OBJECT: i64 = 4;
}


#[repr(C)]
pub struct nsIXPConnect {
    vtable: *const nsIXPConnectVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPConnect {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x768507b5, 0xb981, 0x40c7,
            [0x82, 0x76, 0xf6, 0xa1, 0xda, 0x50, 0x2a, 0x24])
    }
}

unsafe impl RefCounted for nsIXPConnect {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// Enable coercing to ourselves
pub trait nsIXPConnectCoerce {
    fn coerce_from(v: &nsIXPConnect) -> &Self;
}

impl nsIXPConnectCoerce for nsIXPConnect {
    #[inline]
    fn coerce_from(v: &nsIXPConnect) -> &Self {
        v
    }
}

impl nsIXPConnect {
    #[inline]
    pub fn coerce<T: nsIXPConnectCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPConnect {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPConnectCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPConnect) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPConnectVTable {
    pub __base: nsISupportsVTable,

    /* nsIXPConnectJSObjectHolder initClassesWithNewWrappedGlobal (in JSContextPtr aJSContext, in nsISupports aCOMObj, in nsIPrincipal aPrincipal, in uint32_t aFlags, in JSCompartmentOptions aOptions); */
    /// Unable to call function as its signature contains a non-rust type
    pub initClassesWithNewWrappedGlobal: *const ::libc::c_void,

    /* JSObjectPtr wrapNative (in JSContextPtr aJSContext, in JSObjectPtr aScope, in nsISupports aCOMObj, in nsIIDRef aIID); */
    /// Unable to call function as its signature contains a non-rust type
    pub wrapNative: *const ::libc::c_void,

    /* nsIXPConnectJSObjectHolder wrapNativeHolder (in JSContextPtr aJSContext, in JSObjectPtr aScope, in nsISupports aCOMObj, in nsIIDRef aIID); */
    /// Unable to call function as its signature contains a non-rust type
    pub wrapNativeHolder: *const ::libc::c_void,

    /* void wrapNativeToJSVal (in JSContextPtr aJSContext, in JSObjectPtr aScope, in nsISupports aCOMObj, in nsWrapperCachePtr aCache, in nsIIDPtr aIID, in boolean aAllowWrapper, out jsval aVal); */
    /// Unable to call function as its signature contains a non-rust type
    pub wrapNativeToJSVal: *const ::libc::c_void,

    /* void wrapJS (in JSContextPtr aJSContext, in JSObjectPtr aJSObj, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
    /// Unable to call function as its signature contains a non-rust type
    pub wrapJS: *const ::libc::c_void,

    /* nsIVariant jSValToVariant (in JSContextPtr cx, in jsval aJSVal); */
    /// Unable to call function as its signature contains a non-rust type
    pub jSValToVariant: *const ::libc::c_void,

    /* nsIXPConnectWrappedNative getWrappedNativeOfJSObject (in JSContextPtr aJSContext, in JSObjectPtr aJSObj); */
    /// Unable to call function as its signature contains a non-rust type
    pub getWrappedNativeOfJSObject: *const ::libc::c_void,

    /* [noscript,notxpcom] nsISupports getNativeOfWrapper (in JSContextPtr aJSContext, in JSObjectPtr aJSObj); */
    /// Unable to call function as its signature contains a non-rust type
    pub getNativeOfWrapper: *const ::libc::c_void,

    /* readonly attribute nsIStackFrame CurrentJSStack; */
    pub get_CurrentJSStack: unsafe extern "C" fn (this: *const nsIXPConnect, aCurrentJSStack: *mut *const nsIStackFrame) -> nsresult,

    /* readonly attribute nsAXPCNativeCallContextPtr CurrentNativeCallContext; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_CurrentNativeCallContext: *const ::libc::c_void,

    /* void debugDump (in short depth); */
    pub debugDump: unsafe extern "C" fn (this: *const nsIXPConnect, depth: libc::int16_t) -> nsresult,

    /* void debugDumpObject (in nsISupports aCOMObj, in short depth); */
    pub debugDumpObject: unsafe extern "C" fn (this: *const nsIXPConnect, aCOMObj: *const nsISupports, depth: libc::int16_t) -> nsresult,

    /* void debugDumpJSStack (in boolean showArgs, in boolean showLocals, in boolean showThisProps); */
    pub debugDumpJSStack: unsafe extern "C" fn (this: *const nsIXPConnect, showArgs: bool, showLocals: bool, showThisProps: bool) -> nsresult,

    /* void wrapJSAggregatedToNative (in nsISupports aOuter, in JSContextPtr aJSContext, in JSObjectPtr aJSObj, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
    /// Unable to call function as its signature contains a non-rust type
    pub wrapJSAggregatedToNative: *const ::libc::c_void,

    /* nsIXPConnectWrappedNative getWrappedNativeOfNativeObject (in JSContextPtr aJSContext, in JSObjectPtr aScope, in nsISupports aCOMObj, in nsIIDRef aIID); */
    /// Unable to call function as its signature contains a non-rust type
    pub getWrappedNativeOfNativeObject: *const ::libc::c_void,

    /* void setFunctionThisTranslator (in nsIIDRef aIID, in nsIXPCFunctionThisTranslator aTranslator); */
    pub setFunctionThisTranslator: unsafe extern "C" fn (this: *const nsIXPConnect, aIID: *const nsIID, aTranslator: *const nsIXPCFunctionThisTranslator) -> nsresult,

    /* JSObjectPtr getWrappedNativePrototype (in JSContextPtr aJSContext, in JSObjectPtr aScope, in nsIClassInfo aClassInfo); */
    /// Unable to call function as its signature contains a non-rust type
    pub getWrappedNativePrototype: *const ::libc::c_void,

    /* jsval variantToJS (in JSContextPtr ctx, in JSObjectPtr scope, in nsIVariant value); */
    /// Unable to call function as its signature contains a non-rust type
    pub variantToJS: *const ::libc::c_void,

    /* nsIVariant JSToVariant (in JSContextPtr ctx, in jsval value); */
    /// Unable to call function as its signature contains a non-rust type
    pub JSToVariant: *const ::libc::c_void,

    /* [noscript] JSObjectPtr createSandbox (in JSContextPtr cx, in nsIPrincipal principal); */
    /// Unable to call function as its signature contains a non-rust type
    pub createSandbox: *const ::libc::c_void,

    /* [noscript] jsval evalInSandboxObject (in AString source, in string filename, in JSContextPtr cx, in JSObjectPtr sandbox); */
    /// Unable to call function as its signature contains a non-rust type
    pub evalInSandboxObject: *const ::libc::c_void,

    /* void GarbageCollect (in uint32_t reason); */
    pub GarbageCollect: unsafe extern "C" fn (this: *const nsIXPConnect, reason: uint32_t) -> nsresult,

    /* void NotifyDidPaint (); */
    pub NotifyDidPaint: unsafe extern "C" fn (this: *const nsIXPConnect) -> nsresult,

    /* [noscript] void writeScript (in nsIObjectOutputStream aStream, in JSContextPtr aJSContext, in JSScriptPtr aJSScript); */
    /// Unable to call function as its signature contains a non-rust type
    pub writeScript: *const ::libc::c_void,

    /* [noscript] JSScriptPtr readScript (in nsIObjectInputStream aStream, in JSContextPtr aJSContext); */
    /// Unable to call function as its signature contains a non-rust type
    pub readScript: *const ::libc::c_void,

    /* [noscript] void writeFunction (in nsIObjectOutputStream aStream, in JSContextPtr aJSContext, in JSObjectPtr aJSObject); */
    /// Unable to call function as its signature contains a non-rust type
    pub writeFunction: *const ::libc::c_void,

    /* [noscript] JSObjectPtr readFunction (in nsIObjectInputStream aStream, in JSContextPtr aJSContext); */
    /// Unable to call function as its signature contains a non-rust type
    pub readFunction: *const ::libc::c_void,

}


impl nsIXPConnect {
    /* nsIXPConnectJSObjectHolder initClassesWithNewWrappedGlobal (in JSContextPtr aJSContext, in nsISupports aCOMObj, in nsIPrincipal aPrincipal, in uint32_t aFlags, in JSCompartmentOptions aOptions); */


    /* JSObjectPtr wrapNative (in JSContextPtr aJSContext, in JSObjectPtr aScope, in nsISupports aCOMObj, in nsIIDRef aIID); */


    /* nsIXPConnectJSObjectHolder wrapNativeHolder (in JSContextPtr aJSContext, in JSObjectPtr aScope, in nsISupports aCOMObj, in nsIIDRef aIID); */


    /* void wrapNativeToJSVal (in JSContextPtr aJSContext, in JSObjectPtr aScope, in nsISupports aCOMObj, in nsWrapperCachePtr aCache, in nsIIDPtr aIID, in boolean aAllowWrapper, out jsval aVal); */


    /* void wrapJS (in JSContextPtr aJSContext, in JSObjectPtr aJSObj, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */


    /* nsIVariant jSValToVariant (in JSContextPtr cx, in jsval aJSVal); */


    /* nsIXPConnectWrappedNative getWrappedNativeOfJSObject (in JSContextPtr aJSContext, in JSObjectPtr aJSObj); */


    /* [noscript,notxpcom] nsISupports getNativeOfWrapper (in JSContextPtr aJSContext, in JSObjectPtr aJSObj); */


    /* readonly attribute nsIStackFrame CurrentJSStack; */
    #[inline]
    pub unsafe fn get_CurrentJSStack(&self, ) -> Result<Option<RefPtr<nsIStackFrame>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_CurrentJSStack)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsAXPCNativeCallContextPtr CurrentNativeCallContext; */


    /* void debugDump (in short depth); */
    #[inline]
    pub unsafe fn debugDump(&self, depth: libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).debugDump)(self as *const _, depth) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void debugDumpObject (in nsISupports aCOMObj, in short depth); */
    #[inline]
    pub unsafe fn debugDumpObject(&self, aCOMObj: Option<&nsISupports>, depth: libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).debugDumpObject)(self as *const _, aCOMObj.map_or(::std::ptr::null(), |x| x as *const _), depth) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void debugDumpJSStack (in boolean showArgs, in boolean showLocals, in boolean showThisProps); */
    #[inline]
    pub unsafe fn debugDumpJSStack(&self, showArgs: bool, showLocals: bool, showThisProps: bool) -> Result<(), nsresult> {

        match ((*self.vtable).debugDumpJSStack)(self as *const _, showArgs, showLocals, showThisProps) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void wrapJSAggregatedToNative (in nsISupports aOuter, in JSContextPtr aJSContext, in JSObjectPtr aJSObj, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */


    /* nsIXPConnectWrappedNative getWrappedNativeOfNativeObject (in JSContextPtr aJSContext, in JSObjectPtr aScope, in nsISupports aCOMObj, in nsIIDRef aIID); */


    /* void setFunctionThisTranslator (in nsIIDRef aIID, in nsIXPCFunctionThisTranslator aTranslator); */
    #[inline]
    pub unsafe fn setFunctionThisTranslator(&self, aIID: *const nsIID, aTranslator: Option<&nsIXPCFunctionThisTranslator>) -> Result<(), nsresult> {

        match ((*self.vtable).setFunctionThisTranslator)(self as *const _, aIID, aTranslator.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* JSObjectPtr getWrappedNativePrototype (in JSContextPtr aJSContext, in JSObjectPtr aScope, in nsIClassInfo aClassInfo); */


    /* jsval variantToJS (in JSContextPtr ctx, in JSObjectPtr scope, in nsIVariant value); */


    /* nsIVariant JSToVariant (in JSContextPtr ctx, in jsval value); */


    /* [noscript] JSObjectPtr createSandbox (in JSContextPtr cx, in nsIPrincipal principal); */


    /* [noscript] jsval evalInSandboxObject (in AString source, in string filename, in JSContextPtr cx, in JSObjectPtr sandbox); */


    /* void GarbageCollect (in uint32_t reason); */
    #[inline]
    pub unsafe fn GarbageCollect(&self, reason: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).GarbageCollect)(self as *const _, reason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void NotifyDidPaint (); */
    #[inline]
    pub unsafe fn NotifyDidPaint(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).NotifyDidPaint)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void writeScript (in nsIObjectOutputStream aStream, in JSContextPtr aJSContext, in JSScriptPtr aJSScript); */


    /* [noscript] JSScriptPtr readScript (in nsIObjectInputStream aStream, in JSContextPtr aJSContext); */


    /* [noscript] void writeFunction (in nsIObjectOutputStream aStream, in JSContextPtr aJSContext, in JSObjectPtr aJSObject); */


    /* [noscript] JSObjectPtr readFunction (in nsIObjectInputStream aStream, in JSContextPtr aJSContext); */


}


