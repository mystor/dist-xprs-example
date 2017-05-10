//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentURIGrouper.idl
//


#[repr(C)]
pub struct nsIContentURIGrouper {
    vtable: *const nsIContentURIGrouperVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentURIGrouper {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4bb38cb4, 0xc3cb, 0x4d17,
            [0x97, 0x99, 0x1b, 0x31, 0x32, 0xb3, 0x97, 0x23])
    }
}

unsafe impl RefCounted for nsIContentURIGrouper {
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
pub trait nsIContentURIGrouperCoerce {
    fn coerce_from(v: &nsIContentURIGrouper) -> &Self;
}

impl nsIContentURIGrouperCoerce for nsIContentURIGrouper {
    #[inline]
    fn coerce_from(v: &nsIContentURIGrouper) -> &Self {
        v
    }
}

impl nsIContentURIGrouper {
    #[inline]
    pub fn coerce<T: nsIContentURIGrouperCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentURIGrouper {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContentURIGrouperCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentURIGrouper) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentURIGrouperVTable {
    pub __base: nsISupportsVTable,

    /* AString group (in nsIURI aURI); */
    pub group: unsafe extern "C" fn (this: *const nsIContentURIGrouper, aURI: *const nsIURI, _retval: *mut nsAString) -> nsresult,

}


impl nsIContentURIGrouper {
    /* AString group (in nsIURI aURI); */
    #[inline]
    pub unsafe fn group(&self, aURI: Option<&nsIURI>) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).group)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


