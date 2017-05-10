//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/xpcIJSModuleLoader.idl
//


#[repr(C)]
pub struct xpcIJSModuleLoader {
    vtable: *const xpcIJSModuleLoaderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for xpcIJSModuleLoader {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4f94b21f, 0x2920, 0x4bd9,
            [0x82, 0x51, 0x5f, 0xb6, 0x0f, 0xb0, 0x54, 0xb2])
    }
}

unsafe impl RefCounted for xpcIJSModuleLoader {
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
pub trait xpcIJSModuleLoaderCoerce {
    fn coerce_from(v: &xpcIJSModuleLoader) -> &Self;
}

impl xpcIJSModuleLoaderCoerce for xpcIJSModuleLoader {
    #[inline]
    fn coerce_from(v: &xpcIJSModuleLoader) -> &Self {
        v
    }
}

impl xpcIJSModuleLoader {
    #[inline]
    pub fn coerce<T: xpcIJSModuleLoaderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for xpcIJSModuleLoader {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> xpcIJSModuleLoaderCoerce for T {
    #[inline]
    fn coerce_from(v: &xpcIJSModuleLoader) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct xpcIJSModuleLoaderVTable {
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext,optional_argc] jsval import (in AUTF8String aResourceURI, [optional] in jsval targetObj); */
    /// Unable to call function as its signature contains a non-rust type
    pub import: *const ::libc::c_void,

    /* [noscript] JSObjectPtr importInto (in AUTF8String aResourceURI, in JSObjectPtr targetObj, in nsAXPCNativeCallContextPtr cc); */
    /// Unable to call function as its signature contains a non-rust type
    pub importInto: *const ::libc::c_void,

    /* void unload (in AUTF8String aResourceURI); */
    pub unload: unsafe extern "C" fn (this: *const xpcIJSModuleLoader, aResourceURI: *const nsACString) -> nsresult,

    /* boolean isModuleLoaded (in AUTF8String aResourceURI); */
    pub isModuleLoaded: unsafe extern "C" fn (this: *const xpcIJSModuleLoader, aResourceURI: *const nsACString, _retval: *mut bool) -> nsresult,

}


impl xpcIJSModuleLoader {
    /* [implicit_jscontext,optional_argc] jsval import (in AUTF8String aResourceURI, [optional] in jsval targetObj); */


    /* [noscript] JSObjectPtr importInto (in AUTF8String aResourceURI, in JSObjectPtr targetObj, in nsAXPCNativeCallContextPtr cc); */


    /* void unload (in AUTF8String aResourceURI); */
    #[inline]
    pub unsafe fn unload(&self, aResourceURI: &[u8]) -> Result<(), nsresult> {
        let aResourceURI = nsCString::from(aResourceURI);
        match ((*self.vtable).unload)(self as *const _, &*aResourceURI) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isModuleLoaded (in AUTF8String aResourceURI); */
    #[inline]
    pub unsafe fn isModuleLoaded(&self, aResourceURI: &[u8]) -> Result<bool, nsresult> {
        let aResourceURI = nsCString::from(aResourceURI);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isModuleLoaded)(self as *const _, &*aResourceURI, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


