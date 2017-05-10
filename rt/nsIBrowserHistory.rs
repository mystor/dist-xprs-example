//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBrowserHistory.idl
//


#[repr(C)]
pub struct nsIBrowserHistory {
    vtable: *const nsIBrowserHistoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIBrowserHistory {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x20d31479, 0x38de, 0x49f4,
            [0x93, 0x00, 0x56, 0x6d, 0x6e, 0x83, 0x4c, 0x66])
    }
}

unsafe impl RefCounted for nsIBrowserHistory {
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
pub trait nsIBrowserHistoryCoerce {
    fn coerce_from(v: &nsIBrowserHistory) -> &Self;
}

impl nsIBrowserHistoryCoerce for nsIBrowserHistory {
    #[inline]
    fn coerce_from(v: &nsIBrowserHistory) -> &Self {
        v
    }
}

impl nsIBrowserHistory {
    #[inline]
    pub fn coerce<T: nsIBrowserHistoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIBrowserHistory {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIBrowserHistoryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBrowserHistory) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIBrowserHistoryVTable {
    pub __base: nsISupportsVTable,

    /* void removePage (in nsIURI aURI); */
    pub removePage: unsafe extern "C" fn (this: *const nsIBrowserHistory, aURI: *const nsIURI) -> nsresult,

    /* void removePages ([array, size_is (aLength)] in nsIURI aURIs, in unsigned long aLength); */
    /// Unable to call function as its signature contains a non-rust type
    pub removePages: *const ::libc::c_void,

    /* void removePagesFromHost (in AUTF8String aHost, in boolean aEntireDomain); */
    pub removePagesFromHost: unsafe extern "C" fn (this: *const nsIBrowserHistory, aHost: *const nsACString, aEntireDomain: bool) -> nsresult,

    /* void removePagesByTimeframe (in PRTime aBeginTime, in PRTime aEndTime); */
    pub removePagesByTimeframe: unsafe extern "C" fn (this: *const nsIBrowserHistory, aBeginTime: PRTime, aEndTime: PRTime) -> nsresult,

}


impl nsIBrowserHistory {
    /* void removePage (in nsIURI aURI); */
    #[inline]
    pub unsafe fn removePage(&self, aURI: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).removePage)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removePages ([array, size_is (aLength)] in nsIURI aURIs, in unsigned long aLength); */


    /* void removePagesFromHost (in AUTF8String aHost, in boolean aEntireDomain); */
    #[inline]
    pub unsafe fn removePagesFromHost(&self, aHost: &[u8], aEntireDomain: bool) -> Result<(), nsresult> {
        let aHost = nsCString::from(aHost);
        match ((*self.vtable).removePagesFromHost)(self as *const _, &*aHost, aEntireDomain) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removePagesByTimeframe (in PRTime aBeginTime, in PRTime aEndTime); */
    #[inline]
    pub unsafe fn removePagesByTimeframe(&self, aBeginTime: PRTime, aEndTime: PRTime) -> Result<(), nsresult> {

        match ((*self.vtable).removePagesByTimeframe)(self as *const _, aBeginTime, aEndTime) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


