//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUnicharInputStream.idl
//


#[repr(C)]
pub struct nsIUnicharInputStream {
    vtable: *const nsIUnicharInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUnicharInputStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd5e3bd80, 0x6723, 0x4b92,
            [0xb0, 0xc9, 0x22, 0xf6, 0x16, 0x2f, 0xd9, 0x4f])
    }
}

unsafe impl RefCounted for nsIUnicharInputStream {
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
pub trait nsIUnicharInputStreamCoerce {
    fn coerce_from(v: &nsIUnicharInputStream) -> &Self;
}

impl nsIUnicharInputStreamCoerce for nsIUnicharInputStream {
    #[inline]
    fn coerce_from(v: &nsIUnicharInputStream) -> &Self {
        v
    }
}

impl nsIUnicharInputStream {
    #[inline]
    pub fn coerce<T: nsIUnicharInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUnicharInputStream {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUnicharInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUnicharInputStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUnicharInputStreamVTable {
    pub __base: nsISupportsVTable,

    /* [noscript] unsigned long read ([array, size_is (aCount)] in char16_t aBuf, in unsigned long aCount); */
    /// Unable to call function as its signature contains a non-rust type
    pub read: *const ::libc::c_void,

    /* [noscript] unsigned long readSegments (in nsWriteUnicharSegmentFun aWriter, in voidPtr aClosure, in unsigned long aCount); */
    /// Unable to call function as its signature contains a non-rust type
    pub readSegments: *const ::libc::c_void,

    /* unsigned long readString (in unsigned long aCount, out AString aString); */
    pub readString: unsafe extern "C" fn (this: *const nsIUnicharInputStream, aCount: libc::uint32_t, aString: *mut nsAString, _retval: *mut libc::uint32_t) -> nsresult,

    /* void close (); */
    pub close: unsafe extern "C" fn (this: *const nsIUnicharInputStream) -> nsresult,

}


impl nsIUnicharInputStream {
    /* [noscript] unsigned long read ([array, size_is (aCount)] in char16_t aBuf, in unsigned long aCount); */


    /* [noscript] unsigned long readSegments (in nsWriteUnicharSegmentFun aWriter, in voidPtr aClosure, in unsigned long aCount); */


    /* unsigned long readString (in unsigned long aCount, out AString aString); */
    #[inline]
    pub unsafe fn readString(&self, aCount: libc::uint32_t) -> Result<(nsString, libc::uint32_t), nsresult> {
        let mut aString = nsString::new();
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).readString)(self as *const _, aCount, &mut *aString, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aString, _retval))
    }

    /* void close (); */
    #[inline]
    pub unsafe fn close(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).close)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


