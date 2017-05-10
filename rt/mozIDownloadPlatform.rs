//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIDownloadPlatform.idl
//


pub mod mozIDownloadPlatform_consts {
    pub const ZONE_MY_COMPUTER: i64 = 0;
    pub const ZONE_INTRANET: i64 = 1;
    pub const ZONE_TRUSTED: i64 = 2;
    pub const ZONE_INTERNET: i64 = 3;
    pub const ZONE_RESTRICTED: i64 = 4;
}


#[repr(C)]
pub struct mozIDownloadPlatform {
    vtable: *const mozIDownloadPlatformVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIDownloadPlatform {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9f556e4a, 0xd9b3, 0x46c3,
            [0x9f, 0x8f, 0xd0, 0xdb, 0x1a, 0xc6, 0xc8, 0xc1])
    }
}

unsafe impl RefCounted for mozIDownloadPlatform {
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
pub trait mozIDownloadPlatformCoerce {
    fn coerce_from(v: &mozIDownloadPlatform) -> &Self;
}

impl mozIDownloadPlatformCoerce for mozIDownloadPlatform {
    #[inline]
    fn coerce_from(v: &mozIDownloadPlatform) -> &Self {
        v
    }
}

impl mozIDownloadPlatform {
    #[inline]
    pub fn coerce<T: mozIDownloadPlatformCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIDownloadPlatform {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIDownloadPlatformCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIDownloadPlatform) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIDownloadPlatformVTable {
    pub __base: nsISupportsVTable,

    /* void downloadDone (in nsIURI aSource, in nsIURI aReferrer, in nsIFile aTarget, in ACString aContentType, in boolean aIsPrivate); */
    pub downloadDone: unsafe extern "C" fn (this: *const mozIDownloadPlatform, aSource: *const nsIURI, aReferrer: *const nsIURI, aTarget: *const nsIFile, aContentType: *const nsACString, aIsPrivate: bool) -> nsresult,

    /* unsigned long mapUrlToZone (in AString aURL); */
    pub mapUrlToZone: unsafe extern "C" fn (this: *const mozIDownloadPlatform, aURL: *const nsAString, _retval: *mut libc::uint32_t) -> nsresult,

}


impl mozIDownloadPlatform {
    /* void downloadDone (in nsIURI aSource, in nsIURI aReferrer, in nsIFile aTarget, in ACString aContentType, in boolean aIsPrivate); */
    #[inline]
    pub unsafe fn downloadDone(&self, aSource: Option<&nsIURI>, aReferrer: Option<&nsIURI>, aTarget: Option<&nsIFile>, aContentType: &[u8], aIsPrivate: bool) -> Result<(), nsresult> {
        let aContentType = nsCString::from(aContentType);
        match ((*self.vtable).downloadDone)(self as *const _, aSource.map_or(::std::ptr::null(), |x| x as *const _), aReferrer.map_or(::std::ptr::null(), |x| x as *const _), aTarget.map_or(::std::ptr::null(), |x| x as *const _), &*aContentType, aIsPrivate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* unsigned long mapUrlToZone (in AString aURL); */
    #[inline]
    pub unsafe fn mapUrlToZone(&self, aURL: &[u16]) -> Result<libc::uint32_t, nsresult> {
        let aURL = nsString::from(aURL);
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).mapUrlToZone)(self as *const _, &*aURL, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


