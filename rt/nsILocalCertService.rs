//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILocalCertService.idl
//


#[repr(C)]
pub struct nsILocalCertService {
    vtable: *const nsILocalCertServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsILocalCertService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9702fdd4, 0x4c2c, 0x439c,
            [0xba, 0x2e, 0x19, 0xcd, 0xa0, 0x18, 0xeb, 0x99])
    }
}

unsafe impl RefCounted for nsILocalCertService {
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
pub trait nsILocalCertServiceCoerce {
    fn coerce_from(v: &nsILocalCertService) -> &Self;
}

impl nsILocalCertServiceCoerce for nsILocalCertService {
    #[inline]
    fn coerce_from(v: &nsILocalCertService) -> &Self {
        v
    }
}

impl nsILocalCertService {
    #[inline]
    pub fn coerce<T: nsILocalCertServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsILocalCertService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsILocalCertServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILocalCertService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsILocalCertServiceVTable {
    pub __base: nsISupportsVTable,

    /* void getOrCreateCert (in ACString nickname, in nsILocalCertGetCallback cb); */
    pub getOrCreateCert: unsafe extern "C" fn (this: *const nsILocalCertService, nickname: *const nsACString, cb: *const nsILocalCertGetCallback) -> nsresult,

    /* void removeCert (in ACString nickname, in nsILocalCertCallback cb); */
    pub removeCert: unsafe extern "C" fn (this: *const nsILocalCertService, nickname: *const nsACString, cb: *const nsILocalCertCallback) -> nsresult,

    /* readonly attribute boolean loginPromptRequired; */
    pub get_loginPromptRequired: unsafe extern "C" fn (this: *const nsILocalCertService, aLoginPromptRequired: *mut bool) -> nsresult,

}


impl nsILocalCertService {
    /* void getOrCreateCert (in ACString nickname, in nsILocalCertGetCallback cb); */
    #[inline]
    pub unsafe fn getOrCreateCert(&self, nickname: &[u8], cb: Option<&nsILocalCertGetCallback>) -> Result<(), nsresult> {
        let nickname = nsCString::from(nickname);
        match ((*self.vtable).getOrCreateCert)(self as *const _, &*nickname, cb.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeCert (in ACString nickname, in nsILocalCertCallback cb); */
    #[inline]
    pub unsafe fn removeCert(&self, nickname: &[u8], cb: Option<&nsILocalCertCallback>) -> Result<(), nsresult> {
        let nickname = nsCString::from(nickname);
        match ((*self.vtable).removeCert)(self as *const _, &*nickname, cb.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean loginPromptRequired; */
    #[inline]
    pub unsafe fn get_loginPromptRequired(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_loginPromptRequired)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsILocalCertGetCallback {
    vtable: *const nsILocalCertGetCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsILocalCertGetCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xcc09633e, 0x7c70, 0x4093,
            [0xa9, 0xcf, 0x79, 0xab, 0x67, 0x6c, 0xa8, 0xa9])
    }
}

unsafe impl RefCounted for nsILocalCertGetCallback {
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
pub trait nsILocalCertGetCallbackCoerce {
    fn coerce_from(v: &nsILocalCertGetCallback) -> &Self;
}

impl nsILocalCertGetCallbackCoerce for nsILocalCertGetCallback {
    #[inline]
    fn coerce_from(v: &nsILocalCertGetCallback) -> &Self {
        v
    }
}

impl nsILocalCertGetCallback {
    #[inline]
    pub fn coerce<T: nsILocalCertGetCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsILocalCertGetCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsILocalCertGetCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILocalCertGetCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsILocalCertGetCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void handleCert (in nsIX509Cert cert, in nsresult result); */
    pub handleCert: unsafe extern "C" fn (this: *const nsILocalCertGetCallback, cert: *const nsIX509Cert, result: nsresult) -> nsresult,

}


impl nsILocalCertGetCallback {
    /* void handleCert (in nsIX509Cert cert, in nsresult result); */
    #[inline]
    pub unsafe fn handleCert(&self, cert: Option<&nsIX509Cert>, result: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).handleCert)(self as *const _, cert.map_or(::std::ptr::null(), |x| x as *const _), result) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsILocalCertCallback {
    vtable: *const nsILocalCertCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsILocalCertCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x518124e9, 0x55e6, 0x4e23,
            [0x97, 0xc0, 0x49, 0x95, 0xb3, 0xa1, 0xbe, 0xc6])
    }
}

unsafe impl RefCounted for nsILocalCertCallback {
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
pub trait nsILocalCertCallbackCoerce {
    fn coerce_from(v: &nsILocalCertCallback) -> &Self;
}

impl nsILocalCertCallbackCoerce for nsILocalCertCallback {
    #[inline]
    fn coerce_from(v: &nsILocalCertCallback) -> &Self {
        v
    }
}

impl nsILocalCertCallback {
    #[inline]
    pub fn coerce<T: nsILocalCertCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsILocalCertCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsILocalCertCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILocalCertCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsILocalCertCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void handleResult (in nsresult result); */
    pub handleResult: unsafe extern "C" fn (this: *const nsILocalCertCallback, result: nsresult) -> nsresult,

}


impl nsILocalCertCallback {
    /* void handleResult (in nsresult result); */
    #[inline]
    pub unsafe fn handleResult(&self, result: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).handleResult)(self as *const _, result) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


