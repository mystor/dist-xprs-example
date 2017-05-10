//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIColorAnalyzer.idl
//


#[repr(C)]
pub struct mozIRepresentativeColorCallback {
    vtable: *const mozIRepresentativeColorCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIRepresentativeColorCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe4089e21, 0x71b6, 0x40af,
            [0xb5, 0x46, 0x33, 0xc2, 0x1b, 0x90, 0xe8, 0x74])
    }
}

unsafe impl RefCounted for mozIRepresentativeColorCallback {
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
pub trait mozIRepresentativeColorCallbackCoerce {
    fn coerce_from(v: &mozIRepresentativeColorCallback) -> &Self;
}

impl mozIRepresentativeColorCallbackCoerce for mozIRepresentativeColorCallback {
    #[inline]
    fn coerce_from(v: &mozIRepresentativeColorCallback) -> &Self {
        v
    }
}

impl mozIRepresentativeColorCallback {
    #[inline]
    pub fn coerce<T: mozIRepresentativeColorCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIRepresentativeColorCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIRepresentativeColorCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIRepresentativeColorCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIRepresentativeColorCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void onComplete (in boolean success, [optional] in unsigned long color); */
    pub onComplete: unsafe extern "C" fn (this: *const mozIRepresentativeColorCallback, success: bool, color: libc::uint32_t) -> nsresult,

}


impl mozIRepresentativeColorCallback {
    /* void onComplete (in boolean success, [optional] in unsigned long color); */
    #[inline]
    pub unsafe fn onComplete(&self, success: bool, color: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).onComplete)(self as *const _, success, color) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct mozIColorAnalyzer {
    vtable: *const mozIColorAnalyzerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIColorAnalyzer {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd056186c, 0x28a0, 0x494e,
            [0xaa, 0xcc, 0x9e, 0x43, 0x37, 0x72, 0xb1, 0x43])
    }
}

unsafe impl RefCounted for mozIColorAnalyzer {
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
pub trait mozIColorAnalyzerCoerce {
    fn coerce_from(v: &mozIColorAnalyzer) -> &Self;
}

impl mozIColorAnalyzerCoerce for mozIColorAnalyzer {
    #[inline]
    fn coerce_from(v: &mozIColorAnalyzer) -> &Self {
        v
    }
}

impl mozIColorAnalyzer {
    #[inline]
    pub fn coerce<T: mozIColorAnalyzerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIColorAnalyzer {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIColorAnalyzerCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIColorAnalyzer) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIColorAnalyzerVTable {
    pub __base: nsISupportsVTable,

    /* void findRepresentativeColor (in nsIURI imageURI, in mozIRepresentativeColorCallback callback); */
    pub findRepresentativeColor: unsafe extern "C" fn (this: *const mozIColorAnalyzer, imageURI: *const nsIURI, callback: *const mozIRepresentativeColorCallback) -> nsresult,

}


impl mozIColorAnalyzer {
    /* void findRepresentativeColor (in nsIURI imageURI, in mozIRepresentativeColorCallback callback); */
    #[inline]
    pub unsafe fn findRepresentativeColor(&self, imageURI: Option<&nsIURI>, callback: Option<&mozIRepresentativeColorCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).findRepresentativeColor)(self as *const _, imageURI.map_or(::std::ptr::null(), |x| x as *const _), callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


