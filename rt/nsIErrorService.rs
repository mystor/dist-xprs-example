//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIErrorService.idl
//


#[repr(C)]
pub struct nsIErrorService {
    vtable: *const nsIErrorServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIErrorService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xafe1f190, 0xa3c2, 0x11e3,
            [0xa5, 0xe2, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66])
    }
}

unsafe impl RefCounted for nsIErrorService {
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
pub trait nsIErrorServiceCoerce {
    fn coerce_from(v: &nsIErrorService) -> &Self;
}

impl nsIErrorServiceCoerce for nsIErrorService {
    #[inline]
    fn coerce_from(v: &nsIErrorService) -> &Self {
        v
    }
}

impl nsIErrorService {
    #[inline]
    pub fn coerce<T: nsIErrorServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIErrorService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIErrorServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIErrorService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIErrorServiceVTable {
    pub __base: nsISupportsVTable,

    /* void registerErrorStringBundle (in short errorModule, in string stringBundleURL); */
    pub registerErrorStringBundle: unsafe extern "C" fn (this: *const nsIErrorService, errorModule: libc::int16_t, stringBundleURL: *const libc::c_char) -> nsresult,

    /* void unregisterErrorStringBundle (in short errorModule); */
    pub unregisterErrorStringBundle: unsafe extern "C" fn (this: *const nsIErrorService, errorModule: libc::int16_t) -> nsresult,

    /* string getErrorStringBundle (in short errorModule); */
    pub getErrorStringBundle: unsafe extern "C" fn (this: *const nsIErrorService, errorModule: libc::int16_t, _retval: *mut *const libc::c_char) -> nsresult,

}


impl nsIErrorService {
    /* void registerErrorStringBundle (in short errorModule, in string stringBundleURL); */
    #[inline]
    pub unsafe fn registerErrorStringBundle(&self, errorModule: libc::int16_t, stringBundleURL: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).registerErrorStringBundle)(self as *const _, errorModule, stringBundleURL) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unregisterErrorStringBundle (in short errorModule); */
    #[inline]
    pub unsafe fn unregisterErrorStringBundle(&self, errorModule: libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).unregisterErrorStringBundle)(self as *const _, errorModule) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* string getErrorStringBundle (in short errorModule); */
    #[inline]
    pub unsafe fn getErrorStringBundle(&self, errorModule: libc::int16_t) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).getErrorStringBundle)(self as *const _, errorModule, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


