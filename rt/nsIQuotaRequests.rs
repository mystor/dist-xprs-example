//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIQuotaRequests.idl
//


#[repr(C)]
pub struct nsIQuotaRequestBase {
    vtable: *const nsIQuotaRequestBaseVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIQuotaRequestBase {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9af54222, 0x0407, 0x48fd,
            [0xa4, 0xab, 0x94, 0x57, 0xc9, 0x86, 0xfc, 0x49])
    }
}

unsafe impl RefCounted for nsIQuotaRequestBase {
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
pub trait nsIQuotaRequestBaseCoerce {
    fn coerce_from(v: &nsIQuotaRequestBase) -> &Self;
}

impl nsIQuotaRequestBaseCoerce for nsIQuotaRequestBase {
    #[inline]
    fn coerce_from(v: &nsIQuotaRequestBase) -> &Self {
        v
    }
}

impl nsIQuotaRequestBase {
    #[inline]
    pub fn coerce<T: nsIQuotaRequestBaseCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIQuotaRequestBase {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIQuotaRequestBaseCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIQuotaRequestBase) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIQuotaRequestBaseVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIPrincipal principal; */
    pub get_principal: unsafe extern "C" fn (this: *const nsIQuotaRequestBase, aPrincipal: *mut *const nsIPrincipal) -> nsresult,

    /* [must_use] readonly attribute nsresult resultCode; */
    pub get_resultCode: unsafe extern "C" fn (this: *const nsIQuotaRequestBase, aResultCode: *mut nsresult) -> nsresult,

}


impl nsIQuotaRequestBase {
    /* readonly attribute nsIPrincipal principal; */
    #[inline]
    pub unsafe fn get_principal(&self, ) -> Result<Option<RefPtr<nsIPrincipal>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_principal)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [must_use] readonly attribute nsresult resultCode; */
    #[inline]
    pub unsafe fn get_resultCode(&self, ) -> Result<nsresult, nsresult> {
        let mut _retval: nsresult = ::std::mem::zeroed();
        match ((*self.vtable).get_resultCode)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIQuotaUsageRequest {
    vtable: *const nsIQuotaUsageRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIQuotaUsageRequest {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x166e28e6, 0xcf6d, 0x4927,
            [0xa6, 0xd7, 0xb5, 0x1b, 0xca, 0x9d, 0x34, 0x69])
    }
}

unsafe impl RefCounted for nsIQuotaUsageRequest {
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
pub trait nsIQuotaUsageRequestCoerce {
    fn coerce_from(v: &nsIQuotaUsageRequest) -> &Self;
}

impl nsIQuotaUsageRequestCoerce for nsIQuotaUsageRequest {
    #[inline]
    fn coerce_from(v: &nsIQuotaUsageRequest) -> &Self {
        v
    }
}

impl nsIQuotaUsageRequest {
    #[inline]
    pub fn coerce<T: nsIQuotaUsageRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIQuotaUsageRequest {
    type Target = nsIQuotaRequestBase;
    #[inline]
    fn deref(&self) -> &nsIQuotaRequestBase {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIQuotaRequestBaseCoerce> nsIQuotaUsageRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIQuotaUsageRequest) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIQuotaUsageRequestVTable {
    pub __base: nsIQuotaRequestBaseVTable,

    /* [must_use] readonly attribute nsIVariant result; */
    pub get_result: unsafe extern "C" fn (this: *const nsIQuotaUsageRequest, aResult: *mut *const nsIVariant) -> nsresult,

    /* attribute nsIQuotaUsageCallback callback; */
    pub get_callback: unsafe extern "C" fn (this: *const nsIQuotaUsageRequest, aCallback: *mut *const nsIQuotaUsageCallback) -> nsresult,
    pub set_callback: unsafe extern "C" fn (this: *const nsIQuotaUsageRequest, aCallback: *const nsIQuotaUsageCallback) -> nsresult,

    /* [must_use] void cancel (); */
    pub cancel: unsafe extern "C" fn (this: *const nsIQuotaUsageRequest) -> nsresult,

}


impl nsIQuotaUsageRequest {
    /* [must_use] readonly attribute nsIVariant result; */
    #[inline]
    pub unsafe fn get_result(&self, ) -> Result<Option<RefPtr<nsIVariant>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_result)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute nsIQuotaUsageCallback callback; */
    #[inline]
    pub unsafe fn get_callback(&self, ) -> Result<Option<RefPtr<nsIQuotaUsageCallback>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_callback)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_callback(&self, aCallback: Option<&nsIQuotaUsageCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).set_callback)(self as *const _, aCallback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void cancel (); */
    #[inline]
    pub unsafe fn cancel(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).cancel)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIQuotaRequest {
    vtable: *const nsIQuotaRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIQuotaRequest {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x22890e3e, 0xff25, 0x4372,
            [0x96, 0x84, 0xd9, 0x01, 0x06, 0x0e, 0x2f, 0x6c])
    }
}

unsafe impl RefCounted for nsIQuotaRequest {
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
pub trait nsIQuotaRequestCoerce {
    fn coerce_from(v: &nsIQuotaRequest) -> &Self;
}

impl nsIQuotaRequestCoerce for nsIQuotaRequest {
    #[inline]
    fn coerce_from(v: &nsIQuotaRequest) -> &Self {
        v
    }
}

impl nsIQuotaRequest {
    #[inline]
    pub fn coerce<T: nsIQuotaRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIQuotaRequest {
    type Target = nsIQuotaRequestBase;
    #[inline]
    fn deref(&self) -> &nsIQuotaRequestBase {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIQuotaRequestBaseCoerce> nsIQuotaRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIQuotaRequest) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIQuotaRequestVTable {
    pub __base: nsIQuotaRequestBaseVTable,

    /* [must_use] readonly attribute nsIVariant result; */
    pub get_result: unsafe extern "C" fn (this: *const nsIQuotaRequest, aResult: *mut *const nsIVariant) -> nsresult,

    /* attribute nsIQuotaCallback callback; */
    pub get_callback: unsafe extern "C" fn (this: *const nsIQuotaRequest, aCallback: *mut *const nsIQuotaCallback) -> nsresult,
    pub set_callback: unsafe extern "C" fn (this: *const nsIQuotaRequest, aCallback: *const nsIQuotaCallback) -> nsresult,

}


impl nsIQuotaRequest {
    /* [must_use] readonly attribute nsIVariant result; */
    #[inline]
    pub unsafe fn get_result(&self, ) -> Result<Option<RefPtr<nsIVariant>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_result)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute nsIQuotaCallback callback; */
    #[inline]
    pub unsafe fn get_callback(&self, ) -> Result<Option<RefPtr<nsIQuotaCallback>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_callback)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_callback(&self, aCallback: Option<&nsIQuotaCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).set_callback)(self as *const _, aCallback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


