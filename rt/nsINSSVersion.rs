//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINSSVersion.idl
//


#[repr(C)]
pub struct nsINSSVersion {
    vtable: *const nsINSSVersionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINSSVersion {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa8a53a2b, 0x75cc, 0x4c68,
            [0xa9, 0xbb, 0x97, 0x91, 0xdb, 0xdd, 0xaa, 0x00])
    }
}

unsafe impl RefCounted for nsINSSVersion {
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
pub trait nsINSSVersionCoerce {
    fn coerce_from(v: &nsINSSVersion) -> &Self;
}

impl nsINSSVersionCoerce for nsINSSVersion {
    #[inline]
    fn coerce_from(v: &nsINSSVersion) -> &Self {
        v
    }
}

impl nsINSSVersion {
    #[inline]
    pub fn coerce<T: nsINSSVersionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINSSVersion {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINSSVersionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINSSVersion) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINSSVersionVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AString NSPR_MinVersion; */
    pub get_NSPR_MinVersion: unsafe extern "C" fn (this: *const nsINSSVersion, aNSPR_MinVersion: *mut nsAString) -> nsresult,

    /* readonly attribute AString NSS_MinVersion; */
    pub get_NSS_MinVersion: unsafe extern "C" fn (this: *const nsINSSVersion, aNSS_MinVersion: *mut nsAString) -> nsresult,

    /* readonly attribute AString NSSUTIL_MinVersion; */
    pub get_NSSUTIL_MinVersion: unsafe extern "C" fn (this: *const nsINSSVersion, aNSSUTIL_MinVersion: *mut nsAString) -> nsresult,

    /* readonly attribute AString NSSSSL_MinVersion; */
    pub get_NSSSSL_MinVersion: unsafe extern "C" fn (this: *const nsINSSVersion, aNSSSSL_MinVersion: *mut nsAString) -> nsresult,

    /* readonly attribute AString NSSSMIME_MinVersion; */
    pub get_NSSSMIME_MinVersion: unsafe extern "C" fn (this: *const nsINSSVersion, aNSSSMIME_MinVersion: *mut nsAString) -> nsresult,

    /* readonly attribute AString NSPR_Version; */
    pub get_NSPR_Version: unsafe extern "C" fn (this: *const nsINSSVersion, aNSPR_Version: *mut nsAString) -> nsresult,

    /* readonly attribute AString NSS_Version; */
    pub get_NSS_Version: unsafe extern "C" fn (this: *const nsINSSVersion, aNSS_Version: *mut nsAString) -> nsresult,

    /* readonly attribute AString NSSUTIL_Version; */
    pub get_NSSUTIL_Version: unsafe extern "C" fn (this: *const nsINSSVersion, aNSSUTIL_Version: *mut nsAString) -> nsresult,

    /* readonly attribute AString NSSSSL_Version; */
    pub get_NSSSSL_Version: unsafe extern "C" fn (this: *const nsINSSVersion, aNSSSSL_Version: *mut nsAString) -> nsresult,

    /* readonly attribute AString NSSSMIME_Version; */
    pub get_NSSSMIME_Version: unsafe extern "C" fn (this: *const nsINSSVersion, aNSSSMIME_Version: *mut nsAString) -> nsresult,

}


impl nsINSSVersion {
    /* readonly attribute AString NSPR_MinVersion; */
    #[inline]
    pub unsafe fn get_NSPR_MinVersion(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_NSPR_MinVersion)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString NSS_MinVersion; */
    #[inline]
    pub unsafe fn get_NSS_MinVersion(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_NSS_MinVersion)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString NSSUTIL_MinVersion; */
    #[inline]
    pub unsafe fn get_NSSUTIL_MinVersion(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_NSSUTIL_MinVersion)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString NSSSSL_MinVersion; */
    #[inline]
    pub unsafe fn get_NSSSSL_MinVersion(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_NSSSSL_MinVersion)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString NSSSMIME_MinVersion; */
    #[inline]
    pub unsafe fn get_NSSSMIME_MinVersion(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_NSSSMIME_MinVersion)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString NSPR_Version; */
    #[inline]
    pub unsafe fn get_NSPR_Version(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_NSPR_Version)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString NSS_Version; */
    #[inline]
    pub unsafe fn get_NSS_Version(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_NSS_Version)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString NSSUTIL_Version; */
    #[inline]
    pub unsafe fn get_NSSUTIL_Version(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_NSSUTIL_Version)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString NSSSSL_Version; */
    #[inline]
    pub unsafe fn get_NSSSSL_Version(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_NSSSSL_Version)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString NSSSMIME_Version; */
    #[inline]
    pub unsafe fn get_NSSSMIME_Version(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_NSSSMIME_Version)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


