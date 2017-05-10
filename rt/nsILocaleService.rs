//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILocaleService.idl
//


#[repr(C)]
pub struct nsILocaleService {
    vtable: *const nsILocaleServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsILocaleService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc2edc848, 0x4219, 0x4440,
            [0xab, 0xbf, 0x98, 0x11, 0x98, 0x82, 0xc8, 0x3f])
    }
}

unsafe impl RefCounted for nsILocaleService {
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
pub trait nsILocaleServiceCoerce {
    fn coerce_from(v: &nsILocaleService) -> &Self;
}

impl nsILocaleServiceCoerce for nsILocaleService {
    #[inline]
    fn coerce_from(v: &nsILocaleService) -> &Self {
        v
    }
}

impl nsILocaleService {
    #[inline]
    pub fn coerce<T: nsILocaleServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsILocaleService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsILocaleServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILocaleService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsILocaleServiceVTable {
    pub __base: nsISupportsVTable,

    /* nsILocale newLocale (in AString aLocale); */
    pub newLocale: unsafe extern "C" fn (this: *const nsILocaleService, aLocale: *const nsAString, _retval: *mut *const nsILocale) -> nsresult,

    /* nsILocale getSystemLocale (); */
    pub getSystemLocale: unsafe extern "C" fn (this: *const nsILocaleService, _retval: *mut *const nsILocale) -> nsresult,

    /* nsILocale getApplicationLocale (); */
    pub getApplicationLocale: unsafe extern "C" fn (this: *const nsILocaleService, _retval: *mut *const nsILocale) -> nsresult,

    /* nsILocale getLocaleFromAcceptLanguage (in string acceptLanguage); */
    pub getLocaleFromAcceptLanguage: unsafe extern "C" fn (this: *const nsILocaleService, acceptLanguage: *const libc::c_char, _retval: *mut *const nsILocale) -> nsresult,

    /* AString getLocaleComponentForUserAgent (); */
    pub getLocaleComponentForUserAgent: unsafe extern "C" fn (this: *const nsILocaleService, _retval: *mut nsAString) -> nsresult,

}


impl nsILocaleService {
    /* nsILocale newLocale (in AString aLocale); */
    #[inline]
    pub unsafe fn newLocale(&self, aLocale: &[u16]) -> Result<Option<RefPtr<nsILocale>>, nsresult> {
        let aLocale = nsString::from(aLocale);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newLocale)(self as *const _, &*aLocale, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsILocale getSystemLocale (); */
    #[inline]
    pub unsafe fn getSystemLocale(&self, ) -> Result<Option<RefPtr<nsILocale>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getSystemLocale)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsILocale getApplicationLocale (); */
    #[inline]
    pub unsafe fn getApplicationLocale(&self, ) -> Result<Option<RefPtr<nsILocale>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getApplicationLocale)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsILocale getLocaleFromAcceptLanguage (in string acceptLanguage); */
    #[inline]
    pub unsafe fn getLocaleFromAcceptLanguage(&self, acceptLanguage: *const libc::c_char) -> Result<Option<RefPtr<nsILocale>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getLocaleFromAcceptLanguage)(self as *const _, acceptLanguage, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* AString getLocaleComponentForUserAgent (); */
    #[inline]
    pub unsafe fn getLocaleComponentForUserAgent(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getLocaleComponentForUserAgent)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


