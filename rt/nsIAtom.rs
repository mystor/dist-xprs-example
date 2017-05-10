//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAtom.idl
//


#[repr(C)]
pub struct nsIAtom {
    vtable: *const nsIAtomVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAtom {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8b8c11d4, 0x3ed5, 0x4079,
            [0x89, 0x74, 0x73, 0xc7, 0x57, 0x6c, 0xdb, 0x34])
    }
}

unsafe impl RefCounted for nsIAtom {
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
pub trait nsIAtomCoerce {
    fn coerce_from(v: &nsIAtom) -> &Self;
}

impl nsIAtomCoerce for nsIAtom {
    #[inline]
    fn coerce_from(v: &nsIAtom) -> &Self {
        v
    }
}

impl nsIAtom {
    #[inline]
    pub fn coerce<T: nsIAtomCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAtom {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAtomCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAtom) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAtomVTable {
    pub __base: nsISupportsVTable,

    /* [binaryname(ScriptableToString)] AString toString (); */
    pub ScriptableToString: unsafe extern "C" fn (this: *const nsIAtom, _retval: *mut nsAString) -> nsresult,

    /* [noscript] AUTF8String toUTF8String (); */
    pub toUTF8String: unsafe extern "C" fn (this: *const nsIAtom, _retval: *mut nsACString) -> nsresult,

    /* [binaryname(ScriptableEquals)] boolean equals (in AString aString); */
    pub ScriptableEquals: unsafe extern "C" fn (this: *const nsIAtom, aString: *const nsAString, _retval: *mut bool) -> nsresult,

    /* [noscript,notxpcom] size_t SizeOfIncludingThis (in MallocSizeOf aMallocSizeOf); */
    /// Unable to call function as its signature contains a non-rust type
    pub SizeOfIncludingThis: *const ::libc::c_void,

}


impl nsIAtom {
    /* [binaryname(ScriptableToString)] AString toString (); */
    #[inline]
    pub unsafe fn ScriptableToString(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).ScriptableToString)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] AUTF8String toUTF8String (); */
    #[inline]
    pub unsafe fn toUTF8String(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).toUTF8String)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [binaryname(ScriptableEquals)] boolean equals (in AString aString); */
    #[inline]
    pub unsafe fn ScriptableEquals(&self, aString: &[u16]) -> Result<bool, nsresult> {
        let aString = nsString::from(aString);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).ScriptableEquals)(self as *const _, &*aString, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript,notxpcom] size_t SizeOfIncludingThis (in MallocSizeOf aMallocSizeOf); */


}


