//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISensitiveInfoHiddenURI.idl
//


#[repr(C)]
pub struct nsISensitiveInfoHiddenURI {
    vtable: *const nsISensitiveInfoHiddenURIVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISensitiveInfoHiddenURI {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa5761968, 0x6e1a, 0x4f2d,
            [0x81, 0x91, 0xec, 0x74, 0x96, 0x02, 0xb1, 0x78])
    }
}

unsafe impl RefCounted for nsISensitiveInfoHiddenURI {
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
pub trait nsISensitiveInfoHiddenURICoerce {
    fn coerce_from(v: &nsISensitiveInfoHiddenURI) -> &Self;
}

impl nsISensitiveInfoHiddenURICoerce for nsISensitiveInfoHiddenURI {
    #[inline]
    fn coerce_from(v: &nsISensitiveInfoHiddenURI) -> &Self {
        v
    }
}

impl nsISensitiveInfoHiddenURI {
    #[inline]
    pub fn coerce<T: nsISensitiveInfoHiddenURICoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISensitiveInfoHiddenURI {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISensitiveInfoHiddenURICoerce for T {
    #[inline]
    fn coerce_from(v: &nsISensitiveInfoHiddenURI) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISensitiveInfoHiddenURIVTable {
    pub __base: nsISupportsVTable,

    /* AUTF8String getSensitiveInfoHiddenSpec (); */
    pub getSensitiveInfoHiddenSpec: unsafe extern "C" fn (this: *const nsISensitiveInfoHiddenURI, _retval: *mut nsACString) -> nsresult,

}


impl nsISensitiveInfoHiddenURI {
    /* AUTF8String getSensitiveInfoHiddenSpec (); */
    #[inline]
    pub unsafe fn getSensitiveInfoHiddenSpec(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getSensitiveInfoHiddenSpec)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


