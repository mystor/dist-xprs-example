//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPK11TokenDB.idl
//


#[repr(C)]
pub struct nsIPK11TokenDB {
    vtable: *const nsIPK11TokenDBVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPK11TokenDB {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4ee28c82, 0x1dd2, 0x11b2,
            [0xaa, 0xbf, 0xbb, 0x40, 0x17, 0xab, 0xe3, 0x95])
    }
}

unsafe impl RefCounted for nsIPK11TokenDB {
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
pub trait nsIPK11TokenDBCoerce {
    fn coerce_from(v: &nsIPK11TokenDB) -> &Self;
}

impl nsIPK11TokenDBCoerce for nsIPK11TokenDB {
    #[inline]
    fn coerce_from(v: &nsIPK11TokenDB) -> &Self {
        v
    }
}

impl nsIPK11TokenDB {
    #[inline]
    pub fn coerce<T: nsIPK11TokenDBCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPK11TokenDB {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPK11TokenDBCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPK11TokenDB) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPK11TokenDBVTable {
    pub __base: nsISupportsVTable,

    /* nsIPK11Token getInternalKeyToken (); */
    pub getInternalKeyToken: unsafe extern "C" fn (this: *const nsIPK11TokenDB, _retval: *mut *const nsIPK11Token) -> nsresult,

    /* nsIPK11Token findTokenByName (in AUTF8String tokenName); */
    pub findTokenByName: unsafe extern "C" fn (this: *const nsIPK11TokenDB, tokenName: *const nsACString, _retval: *mut *const nsIPK11Token) -> nsresult,

    /* nsISimpleEnumerator listTokens (); */
    pub listTokens: unsafe extern "C" fn (this: *const nsIPK11TokenDB, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

}


impl nsIPK11TokenDB {
    /* nsIPK11Token getInternalKeyToken (); */
    #[inline]
    pub unsafe fn getInternalKeyToken(&self, ) -> Result<Option<RefPtr<nsIPK11Token>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getInternalKeyToken)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIPK11Token findTokenByName (in AUTF8String tokenName); */
    #[inline]
    pub unsafe fn findTokenByName(&self, tokenName: &[u8]) -> Result<Option<RefPtr<nsIPK11Token>>, nsresult> {
        let tokenName = nsCString::from(tokenName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).findTokenByName)(self as *const _, &*tokenName, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsISimpleEnumerator listTokens (); */
    #[inline]
    pub unsafe fn listTokens(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).listTokens)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


