//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIAsyncFavicons.idl
//


#[repr(C)]
pub struct mozIAsyncFavicons {
    vtable: *const mozIAsyncFaviconsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIAsyncFavicons {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa9c81797, 0x9133, 0x4823,
            [0xb5, 0x5f, 0x36, 0x46, 0xe6, 0x7c, 0xfd, 0x41])
    }
}

unsafe impl RefCounted for mozIAsyncFavicons {
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
pub trait mozIAsyncFaviconsCoerce {
    fn coerce_from(v: &mozIAsyncFavicons) -> &Self;
}

impl mozIAsyncFaviconsCoerce for mozIAsyncFavicons {
    #[inline]
    fn coerce_from(v: &mozIAsyncFavicons) -> &Self {
        v
    }
}

impl mozIAsyncFavicons {
    #[inline]
    pub fn coerce<T: mozIAsyncFaviconsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIAsyncFavicons {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIAsyncFaviconsCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIAsyncFavicons) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIAsyncFaviconsVTable {
    pub __base: nsISupportsVTable,

    /* mozIPlacesPendingOperation setAndFetchFaviconForPage (in nsIURI aPageURI, in nsIURI aFaviconURI, in boolean aForceReload, in unsigned long aFaviconLoadType, [optional] in nsIFaviconDataCallback aCallback, [optional] in nsIPrincipal aLoadingPrincipal); */
    pub setAndFetchFaviconForPage: unsafe extern "C" fn (this: *const mozIAsyncFavicons, aPageURI: *const nsIURI, aFaviconURI: *const nsIURI, aForceReload: bool, aFaviconLoadType: libc::uint32_t, aCallback: *const nsIFaviconDataCallback, aLoadingPrincipal: *const nsIPrincipal, _retval: *mut *const mozIPlacesPendingOperation) -> nsresult,

    /* void replaceFaviconData (in nsIURI aFaviconURI, [array, size_is (aDataLen), const] in octet aData, in unsigned long aDataLen, in AUTF8String aMimeType, [optional] in PRTime aExpiration); */
    /// Unable to call function as its signature contains a non-rust type
    pub replaceFaviconData: *const ::libc::c_void,

    /* void replaceFaviconDataFromDataURL (in nsIURI aFaviconURI, in AString aDataURL, [optional] in PRTime aExpiration, [optional] in nsIPrincipal aLoadingPrincipal); */
    pub replaceFaviconDataFromDataURL: unsafe extern "C" fn (this: *const mozIAsyncFavicons, aFaviconURI: *const nsIURI, aDataURL: *const nsAString, aExpiration: PRTime, aLoadingPrincipal: *const nsIPrincipal) -> nsresult,

    /* void getFaviconURLForPage (in nsIURI aPageURI, in nsIFaviconDataCallback aCallback, [optional] in unsigned short aPreferredWidth); */
    pub getFaviconURLForPage: unsafe extern "C" fn (this: *const mozIAsyncFavicons, aPageURI: *const nsIURI, aCallback: *const nsIFaviconDataCallback, aPreferredWidth: libc::uint16_t) -> nsresult,

    /* void getFaviconDataForPage (in nsIURI aPageURI, in nsIFaviconDataCallback aCallback, [optional] in unsigned short aPreferredWidth); */
    pub getFaviconDataForPage: unsafe extern "C" fn (this: *const mozIAsyncFavicons, aPageURI: *const nsIURI, aCallback: *const nsIFaviconDataCallback, aPreferredWidth: libc::uint16_t) -> nsresult,

}


impl mozIAsyncFavicons {
    /* mozIPlacesPendingOperation setAndFetchFaviconForPage (in nsIURI aPageURI, in nsIURI aFaviconURI, in boolean aForceReload, in unsigned long aFaviconLoadType, [optional] in nsIFaviconDataCallback aCallback, [optional] in nsIPrincipal aLoadingPrincipal); */
    #[inline]
    pub unsafe fn setAndFetchFaviconForPage(&self, aPageURI: Option<&nsIURI>, aFaviconURI: Option<&nsIURI>, aForceReload: bool, aFaviconLoadType: libc::uint32_t, aCallback: Option<&nsIFaviconDataCallback>, aLoadingPrincipal: Option<&nsIPrincipal>) -> Result<Option<RefPtr<mozIPlacesPendingOperation>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).setAndFetchFaviconForPage)(self as *const _, aPageURI.map_or(::std::ptr::null(), |x| x as *const _), aFaviconURI.map_or(::std::ptr::null(), |x| x as *const _), aForceReload, aFaviconLoadType, aCallback.map_or(::std::ptr::null(), |x| x as *const _), aLoadingPrincipal.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void replaceFaviconData (in nsIURI aFaviconURI, [array, size_is (aDataLen), const] in octet aData, in unsigned long aDataLen, in AUTF8String aMimeType, [optional] in PRTime aExpiration); */


    /* void replaceFaviconDataFromDataURL (in nsIURI aFaviconURI, in AString aDataURL, [optional] in PRTime aExpiration, [optional] in nsIPrincipal aLoadingPrincipal); */
    #[inline]
    pub unsafe fn replaceFaviconDataFromDataURL(&self, aFaviconURI: Option<&nsIURI>, aDataURL: &[u16], aExpiration: PRTime, aLoadingPrincipal: Option<&nsIPrincipal>) -> Result<(), nsresult> {
        let aDataURL = nsString::from(aDataURL);
        match ((*self.vtable).replaceFaviconDataFromDataURL)(self as *const _, aFaviconURI.map_or(::std::ptr::null(), |x| x as *const _), &*aDataURL, aExpiration, aLoadingPrincipal.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getFaviconURLForPage (in nsIURI aPageURI, in nsIFaviconDataCallback aCallback, [optional] in unsigned short aPreferredWidth); */
    #[inline]
    pub unsafe fn getFaviconURLForPage(&self, aPageURI: Option<&nsIURI>, aCallback: Option<&nsIFaviconDataCallback>, aPreferredWidth: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).getFaviconURLForPage)(self as *const _, aPageURI.map_or(::std::ptr::null(), |x| x as *const _), aCallback.map_or(::std::ptr::null(), |x| x as *const _), aPreferredWidth) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getFaviconDataForPage (in nsIURI aPageURI, in nsIFaviconDataCallback aCallback, [optional] in unsigned short aPreferredWidth); */
    #[inline]
    pub unsafe fn getFaviconDataForPage(&self, aPageURI: Option<&nsIURI>, aCallback: Option<&nsIFaviconDataCallback>, aPreferredWidth: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).getFaviconDataForPage)(self as *const _, aPageURI.map_or(::std::ptr::null(), |x| x as *const _), aCallback.map_or(::std::ptr::null(), |x| x as *const _), aPreferredWidth) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


