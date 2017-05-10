//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIThirdPartyUtil.idl
//


#[repr(C)]
pub struct mozIThirdPartyUtil {
    vtable: *const mozIThirdPartyUtilVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIThirdPartyUtil {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfd82700e, 0xffb4, 0x4932,
            [0xb7, 0xd6, 0x08, 0xf0, 0xb5, 0x69, 0x7d, 0xda])
    }
}

unsafe impl RefCounted for mozIThirdPartyUtil {
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
pub trait mozIThirdPartyUtilCoerce {
    fn coerce_from(v: &mozIThirdPartyUtil) -> &Self;
}

impl mozIThirdPartyUtilCoerce for mozIThirdPartyUtil {
    #[inline]
    fn coerce_from(v: &mozIThirdPartyUtil) -> &Self {
        v
    }
}

impl mozIThirdPartyUtil {
    #[inline]
    pub fn coerce<T: mozIThirdPartyUtilCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIThirdPartyUtil {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIThirdPartyUtilCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIThirdPartyUtil) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIThirdPartyUtilVTable {
    pub __base: nsISupportsVTable,

    /* boolean isThirdPartyURI (in nsIURI aFirstURI, in nsIURI aSecondURI); */
    pub isThirdPartyURI: unsafe extern "C" fn (this: *const mozIThirdPartyUtil, aFirstURI: *const nsIURI, aSecondURI: *const nsIURI, _retval: *mut bool) -> nsresult,

    /* boolean isThirdPartyWindow (in mozIDOMWindowProxy aWindow, [optional] in nsIURI aURI); */
    pub isThirdPartyWindow: unsafe extern "C" fn (this: *const mozIThirdPartyUtil, aWindow: *const mozIDOMWindowProxy, aURI: *const nsIURI, _retval: *mut bool) -> nsresult,

    /* boolean isThirdPartyChannel (in nsIChannel aChannel, [optional] in nsIURI aURI); */
    pub isThirdPartyChannel: unsafe extern "C" fn (this: *const mozIThirdPartyUtil, aChannel: *const nsIChannel, aURI: *const nsIURI, _retval: *mut bool) -> nsresult,

    /* AUTF8String getBaseDomain (in nsIURI aHostURI); */
    pub getBaseDomain: unsafe extern "C" fn (this: *const mozIThirdPartyUtil, aHostURI: *const nsIURI, _retval: *mut nsACString) -> nsresult,

    /* nsIURI getURIFromWindow (in mozIDOMWindowProxy aWindow); */
    pub getURIFromWindow: unsafe extern "C" fn (this: *const mozIThirdPartyUtil, aWindow: *const mozIDOMWindowProxy, _retval: *mut *const nsIURI) -> nsresult,

    /* mozIDOMWindowProxy getTopWindowForChannel (in nsIChannel aChannel); */
    pub getTopWindowForChannel: unsafe extern "C" fn (this: *const mozIThirdPartyUtil, aChannel: *const nsIChannel, _retval: *mut *const mozIDOMWindowProxy) -> nsresult,

}


impl mozIThirdPartyUtil {
    /* boolean isThirdPartyURI (in nsIURI aFirstURI, in nsIURI aSecondURI); */
    #[inline]
    pub unsafe fn isThirdPartyURI(&self, aFirstURI: Option<&nsIURI>, aSecondURI: Option<&nsIURI>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isThirdPartyURI)(self as *const _, aFirstURI.map_or(::std::ptr::null(), |x| x as *const _), aSecondURI.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isThirdPartyWindow (in mozIDOMWindowProxy aWindow, [optional] in nsIURI aURI); */
    #[inline]
    pub unsafe fn isThirdPartyWindow(&self, aWindow: Option<&mozIDOMWindowProxy>, aURI: Option<&nsIURI>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isThirdPartyWindow)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), aURI.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isThirdPartyChannel (in nsIChannel aChannel, [optional] in nsIURI aURI); */
    #[inline]
    pub unsafe fn isThirdPartyChannel(&self, aChannel: Option<&nsIChannel>, aURI: Option<&nsIURI>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isThirdPartyChannel)(self as *const _, aChannel.map_or(::std::ptr::null(), |x| x as *const _), aURI.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AUTF8String getBaseDomain (in nsIURI aHostURI); */
    #[inline]
    pub unsafe fn getBaseDomain(&self, aHostURI: Option<&nsIURI>) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getBaseDomain)(self as *const _, aHostURI.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIURI getURIFromWindow (in mozIDOMWindowProxy aWindow); */
    #[inline]
    pub unsafe fn getURIFromWindow(&self, aWindow: Option<&mozIDOMWindowProxy>) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getURIFromWindow)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* mozIDOMWindowProxy getTopWindowForChannel (in nsIChannel aChannel); */
    #[inline]
    pub unsafe fn getTopWindowForChannel(&self, aChannel: Option<&nsIChannel>) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getTopWindowForChannel)(self as *const _, aChannel.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


