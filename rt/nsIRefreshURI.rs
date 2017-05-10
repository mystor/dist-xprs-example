//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRefreshURI.idl
//


#[repr(C)]
pub struct nsIRefreshURI {
    vtable: *const nsIRefreshURIVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRefreshURI {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa5e61a3c, 0x51bd, 0x45be,
            [0xac, 0x0c, 0xe8, 0x7b, 0x71, 0x86, 0x06, 0x56])
    }
}

unsafe impl RefCounted for nsIRefreshURI {
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
pub trait nsIRefreshURICoerce {
    fn coerce_from(v: &nsIRefreshURI) -> &Self;
}

impl nsIRefreshURICoerce for nsIRefreshURI {
    #[inline]
    fn coerce_from(v: &nsIRefreshURI) -> &Self {
        v
    }
}

impl nsIRefreshURI {
    #[inline]
    pub fn coerce<T: nsIRefreshURICoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRefreshURI {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRefreshURICoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRefreshURI) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRefreshURIVTable {
    pub __base: nsISupportsVTable,

    /* void refreshURI (in nsIURI aURI, in long aMillis, in boolean aRepeat, in boolean aMetaRefresh); */
    pub refreshURI: unsafe extern "C" fn (this: *const nsIRefreshURI, aURI: *const nsIURI, aMillis: libc::int32_t, aRepeat: bool, aMetaRefresh: bool) -> nsresult,

    /* void forceRefreshURI (in nsIURI aURI, in long aMillis, in boolean aMetaRefresh); */
    pub forceRefreshURI: unsafe extern "C" fn (this: *const nsIRefreshURI, aURI: *const nsIURI, aMillis: libc::int32_t, aMetaRefresh: bool) -> nsresult,

    /* void setupRefreshURI (in nsIChannel aChannel); */
    pub setupRefreshURI: unsafe extern "C" fn (this: *const nsIRefreshURI, aChannel: *const nsIChannel) -> nsresult,

    /* void setupRefreshURIFromHeader (in nsIURI aBaseURI, in nsIPrincipal principal, in ACString aHeader); */
    pub setupRefreshURIFromHeader: unsafe extern "C" fn (this: *const nsIRefreshURI, aBaseURI: *const nsIURI, principal: *const nsIPrincipal, aHeader: *const nsACString) -> nsresult,

    /* void cancelRefreshURITimers (); */
    pub cancelRefreshURITimers: unsafe extern "C" fn (this: *const nsIRefreshURI) -> nsresult,

    /* readonly attribute boolean refreshPending; */
    pub get_refreshPending: unsafe extern "C" fn (this: *const nsIRefreshURI, aRefreshPending: *mut bool) -> nsresult,

}


impl nsIRefreshURI {
    /* void refreshURI (in nsIURI aURI, in long aMillis, in boolean aRepeat, in boolean aMetaRefresh); */
    #[inline]
    pub unsafe fn refreshURI(&self, aURI: Option<&nsIURI>, aMillis: libc::int32_t, aRepeat: bool, aMetaRefresh: bool) -> Result<(), nsresult> {

        match ((*self.vtable).refreshURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aMillis, aRepeat, aMetaRefresh) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void forceRefreshURI (in nsIURI aURI, in long aMillis, in boolean aMetaRefresh); */
    #[inline]
    pub unsafe fn forceRefreshURI(&self, aURI: Option<&nsIURI>, aMillis: libc::int32_t, aMetaRefresh: bool) -> Result<(), nsresult> {

        match ((*self.vtable).forceRefreshURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aMillis, aMetaRefresh) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setupRefreshURI (in nsIChannel aChannel); */
    #[inline]
    pub unsafe fn setupRefreshURI(&self, aChannel: Option<&nsIChannel>) -> Result<(), nsresult> {

        match ((*self.vtable).setupRefreshURI)(self as *const _, aChannel.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setupRefreshURIFromHeader (in nsIURI aBaseURI, in nsIPrincipal principal, in ACString aHeader); */
    #[inline]
    pub unsafe fn setupRefreshURIFromHeader(&self, aBaseURI: Option<&nsIURI>, principal: Option<&nsIPrincipal>, aHeader: &[u8]) -> Result<(), nsresult> {
        let aHeader = nsCString::from(aHeader);
        match ((*self.vtable).setupRefreshURIFromHeader)(self as *const _, aBaseURI.map_or(::std::ptr::null(), |x| x as *const _), principal.map_or(::std::ptr::null(), |x| x as *const _), &*aHeader) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void cancelRefreshURITimers (); */
    #[inline]
    pub unsafe fn cancelRefreshURITimers(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).cancelRefreshURITimers)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean refreshPending; */
    #[inline]
    pub unsafe fn get_refreshPending(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_refreshPending)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


