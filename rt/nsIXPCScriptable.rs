//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXPCScriptable.idl
//


#[repr(C)]
pub struct nsIXPCScriptable {
    vtable: *const nsIXPCScriptableVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCScriptable {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x19b70b26, 0x7c3f, 0x437f,
            [0xa0, 0x4a, 0x2a, 0x8f, 0x9e, 0x28, 0xb6, 0x17])
    }
}

unsafe impl RefCounted for nsIXPCScriptable {
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
pub trait nsIXPCScriptableCoerce {
    fn coerce_from(v: &nsIXPCScriptable) -> &Self;
}

impl nsIXPCScriptableCoerce for nsIXPCScriptable {
    #[inline]
    fn coerce_from(v: &nsIXPCScriptable) -> &Self {
        v
    }
}

impl nsIXPCScriptable {
    #[inline]
    pub fn coerce<T: nsIXPCScriptableCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCScriptable {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPCScriptableCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCScriptable) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCScriptableVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute string className; */
    pub get_className: unsafe extern "C" fn (this: *const nsIXPCScriptable, aClassName: *mut *const libc::c_char) -> nsresult,

    /* [nostdcall,notxpcom] uint32_t getScriptableFlags (); */
    pub getScriptableFlags: unsafe extern "C" fn (this: *const nsIXPCScriptable) -> uint32_t,

    /* [nostdcall,notxpcom] jsClassPtr getClass (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getClass: *const ::libc::c_void,

    /* [nostdcall,notxpcom] JSClassPtr getJSClass (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getJSClass: *const ::libc::c_void,

    /* void preCreate (in nsISupports nativeObj, in JSContextPtr cx, in JSObjectPtr globalObj, out JSObjectPtr parentObj); */
    /// Unable to call function as its signature contains a non-rust type
    pub preCreate: *const ::libc::c_void,

    /* boolean getProperty (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in jsid id, in JSValPtr vp); */
    /// Unable to call function as its signature contains a non-rust type
    pub getProperty: *const ::libc::c_void,

    /* boolean setProperty (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in jsid id, in JSValPtr vp); */
    /// Unable to call function as its signature contains a non-rust type
    pub setProperty: *const ::libc::c_void,

    /* boolean enumerate (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj); */
    /// Unable to call function as its signature contains a non-rust type
    pub enumerate: *const ::libc::c_void,

    /* boolean newEnumerate (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in JSAutoIdVector properties); */
    /// Unable to call function as its signature contains a non-rust type
    pub newEnumerate: *const ::libc::c_void,

    /* boolean resolve (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in jsid id, out boolean resolvedp); */
    /// Unable to call function as its signature contains a non-rust type
    pub resolve: *const ::libc::c_void,

    /* void finalize (in nsIXPConnectWrappedNative wrapper, in JSFreeOpPtr fop, in JSObjectPtr obj); */
    /// Unable to call function as its signature contains a non-rust type
    pub finalize: *const ::libc::c_void,

    /* boolean call (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in JSCallArgsRef args); */
    /// Unable to call function as its signature contains a non-rust type
    pub call: *const ::libc::c_void,

    /* boolean construct (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in JSCallArgsRef args); */
    /// Unable to call function as its signature contains a non-rust type
    pub construct: *const ::libc::c_void,

    /* boolean hasInstance (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in jsval val, out boolean bp); */
    /// Unable to call function as its signature contains a non-rust type
    pub hasInstance: *const ::libc::c_void,

    /* void postCreatePrototype (in JSContextPtr cx, in JSObjectPtr proto); */
    /// Unable to call function as its signature contains a non-rust type
    pub postCreatePrototype: *const ::libc::c_void,

}


impl nsIXPCScriptable {
    /* readonly attribute string className; */
    #[inline]
    pub unsafe fn get_className(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_className)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [nostdcall,notxpcom] uint32_t getScriptableFlags (); */
    #[inline]
    pub unsafe fn getScriptableFlags(&self, ) -> uint32_t {

        let _retval = ((*self.vtable).getScriptableFlags)(self as *const _, );
        _retval
    }

    /* [nostdcall,notxpcom] jsClassPtr getClass (); */


    /* [nostdcall,notxpcom] JSClassPtr getJSClass (); */


    /* void preCreate (in nsISupports nativeObj, in JSContextPtr cx, in JSObjectPtr globalObj, out JSObjectPtr parentObj); */


    /* boolean getProperty (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in jsid id, in JSValPtr vp); */


    /* boolean setProperty (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in jsid id, in JSValPtr vp); */


    /* boolean enumerate (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj); */


    /* boolean newEnumerate (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in JSAutoIdVector properties); */


    /* boolean resolve (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in jsid id, out boolean resolvedp); */


    /* void finalize (in nsIXPConnectWrappedNative wrapper, in JSFreeOpPtr fop, in JSObjectPtr obj); */


    /* boolean call (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in JSCallArgsRef args); */


    /* boolean construct (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in JSCallArgsRef args); */


    /* boolean hasInstance (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in jsval val, out boolean bp); */


    /* void postCreatePrototype (in JSContextPtr cx, in JSObjectPtr proto); */


}


