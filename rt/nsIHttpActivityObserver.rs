//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHttpActivityObserver.idl
//


pub mod nsIHttpActivityObserver_consts {
    pub const ACTIVITY_TYPE_SOCKET_TRANSPORT: i64 = 1;
    pub const ACTIVITY_TYPE_HTTP_TRANSACTION: i64 = 2;
    pub const ACTIVITY_SUBTYPE_REQUEST_HEADER: i64 = 20481;
    pub const ACTIVITY_SUBTYPE_REQUEST_BODY_SENT: i64 = 20482;
    pub const ACTIVITY_SUBTYPE_RESPONSE_START: i64 = 20483;
    pub const ACTIVITY_SUBTYPE_RESPONSE_HEADER: i64 = 20484;
    pub const ACTIVITY_SUBTYPE_RESPONSE_COMPLETE: i64 = 20485;
    pub const ACTIVITY_SUBTYPE_TRANSACTION_CLOSE: i64 = 20486;
}


#[repr(C)]
pub struct nsIHttpActivityObserver {
    vtable: *const nsIHttpActivityObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHttpActivityObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x412880c8, 0x6c36, 0x48d8,
            [0xbf, 0x8f, 0x84, 0xf9, 0x1f, 0x89, 0x25, 0x03])
    }
}

unsafe impl RefCounted for nsIHttpActivityObserver {
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
pub trait nsIHttpActivityObserverCoerce {
    fn coerce_from(v: &nsIHttpActivityObserver) -> &Self;
}

impl nsIHttpActivityObserverCoerce for nsIHttpActivityObserver {
    #[inline]
    fn coerce_from(v: &nsIHttpActivityObserver) -> &Self {
        v
    }
}

impl nsIHttpActivityObserver {
    #[inline]
    pub fn coerce<T: nsIHttpActivityObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHttpActivityObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHttpActivityObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpActivityObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHttpActivityObserverVTable {
    pub __base: nsISupportsVTable,

    /* [must_use] void observeActivity (in nsISupports aHttpChannel, in uint32_t aActivityType, in uint32_t aActivitySubtype, in PRTime aTimestamp, in uint64_t aExtraSizeData, in ACString aExtraStringData); */
    pub observeActivity: unsafe extern "C" fn (this: *const nsIHttpActivityObserver, aHttpChannel: *const nsISupports, aActivityType: uint32_t, aActivitySubtype: uint32_t, aTimestamp: PRTime, aExtraSizeData: uint64_t, aExtraStringData: *const nsACString) -> nsresult,

    /* [must_use] readonly attribute boolean isActive; */
    pub get_isActive: unsafe extern "C" fn (this: *const nsIHttpActivityObserver, aIsActive: *mut bool) -> nsresult,

}


impl nsIHttpActivityObserver {
    /* [must_use] void observeActivity (in nsISupports aHttpChannel, in uint32_t aActivityType, in uint32_t aActivitySubtype, in PRTime aTimestamp, in uint64_t aExtraSizeData, in ACString aExtraStringData); */
    #[inline]
    pub unsafe fn observeActivity(&self, aHttpChannel: Option<&nsISupports>, aActivityType: uint32_t, aActivitySubtype: uint32_t, aTimestamp: PRTime, aExtraSizeData: uint64_t, aExtraStringData: &[u8]) -> Result<(), nsresult> {
        let aExtraStringData = nsCString::from(aExtraStringData);
        match ((*self.vtable).observeActivity)(self as *const _, aHttpChannel.map_or(::std::ptr::null(), |x| x as *const _), aActivityType, aActivitySubtype, aTimestamp, aExtraSizeData, &*aExtraStringData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] readonly attribute boolean isActive; */
    #[inline]
    pub unsafe fn get_isActive(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isActive)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIHttpActivityDistributor {
    vtable: *const nsIHttpActivityDistributorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHttpActivityDistributor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7c512cb8, 0x582a, 0x4625,
            [0xb5, 0xb6, 0x86, 0x39, 0x75, 0x52, 0x71, 0xb5])
    }
}

unsafe impl RefCounted for nsIHttpActivityDistributor {
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
pub trait nsIHttpActivityDistributorCoerce {
    fn coerce_from(v: &nsIHttpActivityDistributor) -> &Self;
}

impl nsIHttpActivityDistributorCoerce for nsIHttpActivityDistributor {
    #[inline]
    fn coerce_from(v: &nsIHttpActivityDistributor) -> &Self {
        v
    }
}

impl nsIHttpActivityDistributor {
    #[inline]
    pub fn coerce<T: nsIHttpActivityDistributorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHttpActivityDistributor {
    type Target = nsIHttpActivityObserver;
    #[inline]
    fn deref(&self) -> &nsIHttpActivityObserver {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIHttpActivityObserverCoerce> nsIHttpActivityDistributorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpActivityDistributor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHttpActivityDistributorVTable {
    pub __base: nsIHttpActivityObserverVTable,

    /* void addObserver (in nsIHttpActivityObserver aObserver); */
    pub addObserver: unsafe extern "C" fn (this: *const nsIHttpActivityDistributor, aObserver: *const nsIHttpActivityObserver) -> nsresult,

    /* void removeObserver (in nsIHttpActivityObserver aObserver); */
    pub removeObserver: unsafe extern "C" fn (this: *const nsIHttpActivityDistributor, aObserver: *const nsIHttpActivityObserver) -> nsresult,

}


impl nsIHttpActivityDistributor {
    /* void addObserver (in nsIHttpActivityObserver aObserver); */
    #[inline]
    pub unsafe fn addObserver(&self, aObserver: Option<&nsIHttpActivityObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).addObserver)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeObserver (in nsIHttpActivityObserver aObserver); */
    #[inline]
    pub unsafe fn removeObserver(&self, aObserver: Option<&nsIHttpActivityObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).removeObserver)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


