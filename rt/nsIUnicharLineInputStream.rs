//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUnicharLineInputStream.idl
//


#[repr(C)]
pub struct nsIUnicharLineInputStream {
    vtable: *const nsIUnicharLineInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUnicharLineInputStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x67f42475, 0xba80, 0x40f8,
            [0xac, 0x0b, 0x64, 0x9c, 0x89, 0x23, 0x01, 0x84])
    }
}

unsafe impl RefCounted for nsIUnicharLineInputStream {
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
pub trait nsIUnicharLineInputStreamCoerce {
    fn coerce_from(v: &nsIUnicharLineInputStream) -> &Self;
}

impl nsIUnicharLineInputStreamCoerce for nsIUnicharLineInputStream {
    #[inline]
    fn coerce_from(v: &nsIUnicharLineInputStream) -> &Self {
        v
    }
}

impl nsIUnicharLineInputStream {
    #[inline]
    pub fn coerce<T: nsIUnicharLineInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUnicharLineInputStream {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUnicharLineInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUnicharLineInputStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUnicharLineInputStreamVTable {
    pub __base: nsISupportsVTable,

    /* boolean readLine (out AString aLine); */
    pub readLine: unsafe extern "C" fn (this: *const nsIUnicharLineInputStream, aLine: *mut nsAString, _retval: *mut bool) -> nsresult,

}


impl nsIUnicharLineInputStream {
    /* boolean readLine (out AString aLine); */
    #[inline]
    pub unsafe fn readLine(&self, ) -> Result<(nsString, bool), nsresult> {
        let mut aLine = nsString::new();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).readLine)(self as *const _, &mut *aLine, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aLine, _retval))
    }

}


