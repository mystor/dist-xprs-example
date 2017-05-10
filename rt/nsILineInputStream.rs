//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILineInputStream.idl
//


#[repr(C)]
pub struct nsILineInputStream {
    vtable: *const nsILineInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsILineInputStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc97b466c, 0x1e6e, 0x4773,
            [0xa4, 0xab, 0x2b, 0x2b, 0x31, 0x90, 0xa7, 0xa6])
    }
}

unsafe impl RefCounted for nsILineInputStream {
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
pub trait nsILineInputStreamCoerce {
    fn coerce_from(v: &nsILineInputStream) -> &Self;
}

impl nsILineInputStreamCoerce for nsILineInputStream {
    #[inline]
    fn coerce_from(v: &nsILineInputStream) -> &Self {
        v
    }
}

impl nsILineInputStream {
    #[inline]
    pub fn coerce<T: nsILineInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsILineInputStream {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsILineInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILineInputStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsILineInputStreamVTable {
    pub __base: nsISupportsVTable,

    /* boolean readLine (out ACString aLine); */
    pub readLine: unsafe extern "C" fn (this: *const nsILineInputStream, aLine: *mut nsACString, _retval: *mut bool) -> nsresult,

}


impl nsILineInputStream {
    /* boolean readLine (out ACString aLine); */
    #[inline]
    pub unsafe fn readLine(&self, ) -> Result<(nsCString, bool), nsresult> {
        let mut aLine = nsCString::new();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).readLine)(self as *const _, &mut *aLine, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aLine, _retval))
    }

}


