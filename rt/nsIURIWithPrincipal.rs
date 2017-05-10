//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIURIWithPrincipal.idl
//


#[repr(C)]
pub struct nsIURIWithPrincipal {
    vtable: *const nsIURIWithPrincipalVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIURIWithPrincipal {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x626a5c0c, 0xbfd8, 0x4531,
            [0x8b, 0x47, 0xa8, 0x45, 0x1b, 0x0d, 0xaa, 0x33])
    }
}

unsafe impl RefCounted for nsIURIWithPrincipal {
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
pub trait nsIURIWithPrincipalCoerce {
    fn coerce_from(v: &nsIURIWithPrincipal) -> &Self;
}

impl nsIURIWithPrincipalCoerce for nsIURIWithPrincipal {
    #[inline]
    fn coerce_from(v: &nsIURIWithPrincipal) -> &Self {
        v
    }
}

impl nsIURIWithPrincipal {
    #[inline]
    pub fn coerce<T: nsIURIWithPrincipalCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIURIWithPrincipal {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIURIWithPrincipalCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIURIWithPrincipal) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIURIWithPrincipalVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIPrincipal principal; */
    pub get_principal: unsafe extern "C" fn (this: *const nsIURIWithPrincipal, aPrincipal: *mut *const nsIPrincipal) -> nsresult,

    /* readonly attribute nsIURI principalUri; */
    pub get_principalUri: unsafe extern "C" fn (this: *const nsIURIWithPrincipal, aPrincipalUri: *mut *const nsIURI) -> nsresult,

}


impl nsIURIWithPrincipal {
    /* readonly attribute nsIPrincipal principal; */
    #[inline]
    pub unsafe fn get_principal(&self, ) -> Result<Option<RefPtr<nsIPrincipal>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_principal)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIURI principalUri; */
    #[inline]
    pub unsafe fn get_principalUri(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_principalUri)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


