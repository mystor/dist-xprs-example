//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIX509CertValidity.idl
//


#[repr(C)]
pub struct nsIX509CertValidity {
    vtable: *const nsIX509CertValidityVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIX509CertValidity {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe701dfd8, 0x1dd1, 0x11b2,
            [0xa1, 0x72, 0xff, 0xa6, 0xcc, 0x61, 0x56, 0xad])
    }
}

unsafe impl RefCounted for nsIX509CertValidity {
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
pub trait nsIX509CertValidityCoerce {
    fn coerce_from(v: &nsIX509CertValidity) -> &Self;
}

impl nsIX509CertValidityCoerce for nsIX509CertValidity {
    #[inline]
    fn coerce_from(v: &nsIX509CertValidity) -> &Self {
        v
    }
}

impl nsIX509CertValidity {
    #[inline]
    pub fn coerce<T: nsIX509CertValidityCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIX509CertValidity {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIX509CertValidityCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIX509CertValidity) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIX509CertValidityVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute PRTime notBefore; */
    pub get_notBefore: unsafe extern "C" fn (this: *const nsIX509CertValidity, aNotBefore: *mut PRTime) -> nsresult,

    /* readonly attribute AString notBeforeLocalTime; */
    pub get_notBeforeLocalTime: unsafe extern "C" fn (this: *const nsIX509CertValidity, aNotBeforeLocalTime: *mut nsAString) -> nsresult,

    /* readonly attribute AString notBeforeLocalDay; */
    pub get_notBeforeLocalDay: unsafe extern "C" fn (this: *const nsIX509CertValidity, aNotBeforeLocalDay: *mut nsAString) -> nsresult,

    /* readonly attribute AString notBeforeGMT; */
    pub get_notBeforeGMT: unsafe extern "C" fn (this: *const nsIX509CertValidity, aNotBeforeGMT: *mut nsAString) -> nsresult,

    /* readonly attribute PRTime notAfter; */
    pub get_notAfter: unsafe extern "C" fn (this: *const nsIX509CertValidity, aNotAfter: *mut PRTime) -> nsresult,

    /* readonly attribute AString notAfterLocalTime; */
    pub get_notAfterLocalTime: unsafe extern "C" fn (this: *const nsIX509CertValidity, aNotAfterLocalTime: *mut nsAString) -> nsresult,

    /* readonly attribute AString notAfterLocalDay; */
    pub get_notAfterLocalDay: unsafe extern "C" fn (this: *const nsIX509CertValidity, aNotAfterLocalDay: *mut nsAString) -> nsresult,

    /* readonly attribute AString notAfterGMT; */
    pub get_notAfterGMT: unsafe extern "C" fn (this: *const nsIX509CertValidity, aNotAfterGMT: *mut nsAString) -> nsresult,

}


impl nsIX509CertValidity {
    /* readonly attribute PRTime notBefore; */
    #[inline]
    pub unsafe fn get_notBefore(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_notBefore)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString notBeforeLocalTime; */
    #[inline]
    pub unsafe fn get_notBeforeLocalTime(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_notBeforeLocalTime)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString notBeforeLocalDay; */
    #[inline]
    pub unsafe fn get_notBeforeLocalDay(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_notBeforeLocalDay)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString notBeforeGMT; */
    #[inline]
    pub unsafe fn get_notBeforeGMT(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_notBeforeGMT)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime notAfter; */
    #[inline]
    pub unsafe fn get_notAfter(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_notAfter)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString notAfterLocalTime; */
    #[inline]
    pub unsafe fn get_notAfterLocalTime(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_notAfterLocalTime)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString notAfterLocalDay; */
    #[inline]
    pub unsafe fn get_notAfterLocalDay(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_notAfterLocalDay)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString notAfterGMT; */
    #[inline]
    pub unsafe fn get_notAfterGMT(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_notAfterGMT)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


