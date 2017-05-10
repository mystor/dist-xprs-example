//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICookieManager.idl
//


#[repr(C)]
pub struct nsIPrivateModeCallback {
    vtable: *const nsIPrivateModeCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPrivateModeCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x20709db8, 0x8dad, 0x4e45,
            [0xb3, 0x3e, 0x6e, 0x7c, 0x76, 0x1d, 0xfc, 0x5d])
    }
}

unsafe impl RefCounted for nsIPrivateModeCallback {
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
pub trait nsIPrivateModeCallbackCoerce {
    fn coerce_from(v: &nsIPrivateModeCallback) -> &Self;
}

impl nsIPrivateModeCallbackCoerce for nsIPrivateModeCallback {
    #[inline]
    fn coerce_from(v: &nsIPrivateModeCallback) -> &Self {
        v
    }
}

impl nsIPrivateModeCallback {
    #[inline]
    pub fn coerce<T: nsIPrivateModeCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPrivateModeCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPrivateModeCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrivateModeCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPrivateModeCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void callback (); */
    pub callback: unsafe extern "C" fn (this: *const nsIPrivateModeCallback) -> nsresult,

}


impl nsIPrivateModeCallback {
    /* void callback (); */
    #[inline]
    pub unsafe fn callback(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).callback)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsICookieManager {
    vtable: *const nsICookieManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICookieManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xaaab6710, 0x0f2c, 0x11d5,
            [0xa5, 0x3b, 0x00, 0x10, 0xa4, 0x01, 0xeb, 0x10])
    }
}

unsafe impl RefCounted for nsICookieManager {
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
pub trait nsICookieManagerCoerce {
    fn coerce_from(v: &nsICookieManager) -> &Self;
}

impl nsICookieManagerCoerce for nsICookieManager {
    #[inline]
    fn coerce_from(v: &nsICookieManager) -> &Self {
        v
    }
}

impl nsICookieManager {
    #[inline]
    pub fn coerce<T: nsICookieManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICookieManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICookieManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICookieManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICookieManagerVTable {
    pub __base: nsISupportsVTable,

    /* void removeAll (); */
    pub removeAll: unsafe extern "C" fn (this: *const nsICookieManager) -> nsresult,

    /* readonly attribute nsISimpleEnumerator enumerator; */
    pub get_enumerator: unsafe extern "C" fn (this: *const nsICookieManager, aEnumerator: *mut *const nsISimpleEnumerator) -> nsresult,

    /* [implicit_jscontext,optional_argc] void remove (in AUTF8String aHost, in ACString aName, in AUTF8String aPath, in boolean aBlocked, [optional] in jsval aOriginAttributes); */
    /// Unable to call function as its signature contains a non-rust type
    pub remove: *const ::libc::c_void,

    /* [notxpcom] nsresult removeNative (in AUTF8String aHost, in ACString aName, in AUTF8String aPath, in boolean aBlocked, in OriginAttributesPtr aOriginAttributes); */
    /// Unable to call function as its signature contains a non-rust type
    pub removeNative: *const ::libc::c_void,

    /* void usePrivateMode (in boolean aIsPrivate, in nsIPrivateModeCallback aCallback); */
    pub usePrivateMode: unsafe extern "C" fn (this: *const nsICookieManager, aIsPrivate: bool, aCallback: *const nsIPrivateModeCallback) -> nsresult,

}


impl nsICookieManager {
    /* void removeAll (); */
    #[inline]
    pub unsafe fn removeAll(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).removeAll)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsISimpleEnumerator enumerator; */
    #[inline]
    pub unsafe fn get_enumerator(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_enumerator)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [implicit_jscontext,optional_argc] void remove (in AUTF8String aHost, in ACString aName, in AUTF8String aPath, in boolean aBlocked, [optional] in jsval aOriginAttributes); */


    /* [notxpcom] nsresult removeNative (in AUTF8String aHost, in ACString aName, in AUTF8String aPath, in boolean aBlocked, in OriginAttributesPtr aOriginAttributes); */


    /* void usePrivateMode (in boolean aIsPrivate, in nsIPrivateModeCallback aCallback); */
    #[inline]
    pub unsafe fn usePrivateMode(&self, aIsPrivate: bool, aCallback: Option<&nsIPrivateModeCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).usePrivateMode)(self as *const _, aIsPrivate, aCallback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


