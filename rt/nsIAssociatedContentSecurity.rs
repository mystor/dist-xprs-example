//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAssociatedContentSecurity.idl
//


#[repr(C)]
pub struct nsIAssociatedContentSecurity {
    vtable: *const nsIAssociatedContentSecurityVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAssociatedContentSecurity {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa8285dae, 0xf125, 0x454f,
            [0x9d, 0x1b, 0x08, 0x9e, 0x3f, 0x01, 0xb2, 0xc4])
    }
}

unsafe impl RefCounted for nsIAssociatedContentSecurity {
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
pub trait nsIAssociatedContentSecurityCoerce {
    fn coerce_from(v: &nsIAssociatedContentSecurity) -> &Self;
}

impl nsIAssociatedContentSecurityCoerce for nsIAssociatedContentSecurity {
    #[inline]
    fn coerce_from(v: &nsIAssociatedContentSecurity) -> &Self {
        v
    }
}

impl nsIAssociatedContentSecurity {
    #[inline]
    pub fn coerce<T: nsIAssociatedContentSecurityCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAssociatedContentSecurity {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAssociatedContentSecurityCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAssociatedContentSecurity) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAssociatedContentSecurityVTable {
    pub __base: nsISupportsVTable,

    /* attribute long countSubRequestsBrokenSecurity; */
    pub get_countSubRequestsBrokenSecurity: unsafe extern "C" fn (this: *const nsIAssociatedContentSecurity, aCountSubRequestsBrokenSecurity: *mut libc::int32_t) -> nsresult,
    pub set_countSubRequestsBrokenSecurity: unsafe extern "C" fn (this: *const nsIAssociatedContentSecurity, aCountSubRequestsBrokenSecurity: libc::int32_t) -> nsresult,

    /* attribute long countSubRequestsNoSecurity; */
    pub get_countSubRequestsNoSecurity: unsafe extern "C" fn (this: *const nsIAssociatedContentSecurity, aCountSubRequestsNoSecurity: *mut libc::int32_t) -> nsresult,
    pub set_countSubRequestsNoSecurity: unsafe extern "C" fn (this: *const nsIAssociatedContentSecurity, aCountSubRequestsNoSecurity: libc::int32_t) -> nsresult,

    /* void flush (); */
    pub flush: unsafe extern "C" fn (this: *const nsIAssociatedContentSecurity) -> nsresult,

}


impl nsIAssociatedContentSecurity {
    /* attribute long countSubRequestsBrokenSecurity; */
    #[inline]
    pub unsafe fn get_countSubRequestsBrokenSecurity(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_countSubRequestsBrokenSecurity)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_countSubRequestsBrokenSecurity(&self, aCountSubRequestsBrokenSecurity: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_countSubRequestsBrokenSecurity)(self as *const _, aCountSubRequestsBrokenSecurity) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long countSubRequestsNoSecurity; */
    #[inline]
    pub unsafe fn get_countSubRequestsNoSecurity(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_countSubRequestsNoSecurity)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_countSubRequestsNoSecurity(&self, aCountSubRequestsNoSecurity: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_countSubRequestsNoSecurity)(self as *const _, aCountSubRequestsNoSecurity) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void flush (); */
    #[inline]
    pub unsafe fn flush(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).flush)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


