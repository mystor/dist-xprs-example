//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISubstitutingProtocolHandler.idl
//


#[repr(C)]
pub struct nsISubstitutingProtocolHandler {
    vtable: *const nsISubstitutingProtocolHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISubstitutingProtocolHandler {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x154c64fd, 0xa69e, 0x4105,
            [0x89, 0xf8, 0xbd, 0x7d, 0xfe, 0x62, 0x13, 0x72])
    }
}

unsafe impl RefCounted for nsISubstitutingProtocolHandler {
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
pub trait nsISubstitutingProtocolHandlerCoerce {
    fn coerce_from(v: &nsISubstitutingProtocolHandler) -> &Self;
}

impl nsISubstitutingProtocolHandlerCoerce for nsISubstitutingProtocolHandler {
    #[inline]
    fn coerce_from(v: &nsISubstitutingProtocolHandler) -> &Self {
        v
    }
}

impl nsISubstitutingProtocolHandler {
    #[inline]
    pub fn coerce<T: nsISubstitutingProtocolHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISubstitutingProtocolHandler {
    type Target = nsIProtocolHandler;
    #[inline]
    fn deref(&self) -> &nsIProtocolHandler {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIProtocolHandlerCoerce> nsISubstitutingProtocolHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISubstitutingProtocolHandler) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISubstitutingProtocolHandlerVTable {
    pub __base: nsIProtocolHandlerVTable,

    /* [must_use] void setSubstitution (in ACString root, in nsIURI baseURI); */
    pub setSubstitution: unsafe extern "C" fn (this: *const nsISubstitutingProtocolHandler, root: *const nsACString, baseURI: *const nsIURI) -> nsresult,

    /* [must_use] nsIURI getSubstitution (in ACString root); */
    pub getSubstitution: unsafe extern "C" fn (this: *const nsISubstitutingProtocolHandler, root: *const nsACString, _retval: *mut *const nsIURI) -> nsresult,

    /* [must_use] boolean hasSubstitution (in ACString root); */
    pub hasSubstitution: unsafe extern "C" fn (this: *const nsISubstitutingProtocolHandler, root: *const nsACString, _retval: *mut bool) -> nsresult,

    /* [must_use] AUTF8String resolveURI (in nsIURI resURI); */
    pub resolveURI: unsafe extern "C" fn (this: *const nsISubstitutingProtocolHandler, resURI: *const nsIURI, _retval: *mut nsACString) -> nsresult,

}


impl nsISubstitutingProtocolHandler {
    /* [must_use] void setSubstitution (in ACString root, in nsIURI baseURI); */
    #[inline]
    pub unsafe fn setSubstitution(&self, root: &[u8], baseURI: Option<&nsIURI>) -> Result<(), nsresult> {
        let root = nsCString::from(root);
        match ((*self.vtable).setSubstitution)(self as *const _, &*root, baseURI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] nsIURI getSubstitution (in ACString root); */
    #[inline]
    pub unsafe fn getSubstitution(&self, root: &[u8]) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let root = nsCString::from(root);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getSubstitution)(self as *const _, &*root, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [must_use] boolean hasSubstitution (in ACString root); */
    #[inline]
    pub unsafe fn hasSubstitution(&self, root: &[u8]) -> Result<bool, nsresult> {
        let root = nsCString::from(root);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasSubstitution)(self as *const _, &*root, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] AUTF8String resolveURI (in nsIURI resURI); */
    #[inline]
    pub unsafe fn resolveURI(&self, resURI: Option<&nsIURI>) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).resolveURI)(self as *const _, resURI.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


