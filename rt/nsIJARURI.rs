//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIJARURI.idl
//


#[repr(C)]
pub struct nsIJARURI {
    vtable: *const nsIJARURIVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIJARURI {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x646a508c, 0xf786, 0x4e14,
            [0xbe, 0x6d, 0x8d, 0xda, 0x2a, 0x63, 0x3c, 0x60])
    }
}

unsafe impl RefCounted for nsIJARURI {
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
pub trait nsIJARURICoerce {
    fn coerce_from(v: &nsIJARURI) -> &Self;
}

impl nsIJARURICoerce for nsIJARURI {
    #[inline]
    fn coerce_from(v: &nsIJARURI) -> &Self {
        v
    }
}

impl nsIJARURI {
    #[inline]
    pub fn coerce<T: nsIJARURICoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIJARURI {
    type Target = nsIURL;
    #[inline]
    fn deref(&self) -> &nsIURL {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIURLCoerce> nsIJARURICoerce for T {
    #[inline]
    fn coerce_from(v: &nsIJARURI) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIJARURIVTable {
    pub __base: nsIURLVTable,

    /* readonly attribute nsIURI JARFile; */
    pub get_JARFile: unsafe extern "C" fn (this: *const nsIJARURI, aJARFile: *mut *const nsIURI) -> nsresult,

    /* attribute AUTF8String JAREntry; */
    pub get_JAREntry: unsafe extern "C" fn (this: *const nsIJARURI, aJAREntry: *mut nsACString) -> nsresult,
    pub set_JAREntry: unsafe extern "C" fn (this: *const nsIJARURI, aJAREntry: *const nsACString) -> nsresult,

    /* nsIJARURI cloneWithJARFile (in nsIURI jarFile); */
    pub cloneWithJARFile: unsafe extern "C" fn (this: *const nsIJARURI, jarFile: *const nsIURI, _retval: *mut *const nsIJARURI) -> nsresult,

}


impl nsIJARURI {
    /* readonly attribute nsIURI JARFile; */
    #[inline]
    pub unsafe fn get_JARFile(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_JARFile)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute AUTF8String JAREntry; */
    #[inline]
    pub unsafe fn get_JAREntry(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_JAREntry)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_JAREntry(&self, aJAREntry: &[u8]) -> Result<(), nsresult> {
        let aJAREntry = nsCString::from(aJAREntry);
        match ((*self.vtable).set_JAREntry)(self as *const _, &*aJAREntry) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIJARURI cloneWithJARFile (in nsIURI jarFile); */
    #[inline]
    pub unsafe fn cloneWithJARFile(&self, jarFile: Option<&nsIURI>) -> Result<Option<RefPtr<nsIJARURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).cloneWithJARFile)(self as *const _, jarFile.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


