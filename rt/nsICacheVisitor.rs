//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICacheVisitor.idl
//


#[repr(C)]
pub struct nsICacheVisitor {
    vtable: *const nsICacheVisitorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICacheVisitor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf8c08c4b, 0xd778, 0x49d1,
            [0xa5, 0x9b, 0x86, 0x6f, 0xdc, 0x50, 0x0d, 0x95])
    }
}

unsafe impl RefCounted for nsICacheVisitor {
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
pub trait nsICacheVisitorCoerce {
    fn coerce_from(v: &nsICacheVisitor) -> &Self;
}

impl nsICacheVisitorCoerce for nsICacheVisitor {
    #[inline]
    fn coerce_from(v: &nsICacheVisitor) -> &Self {
        v
    }
}

impl nsICacheVisitor {
    #[inline]
    pub fn coerce<T: nsICacheVisitorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICacheVisitor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICacheVisitorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheVisitor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICacheVisitorVTable {
    pub __base: nsISupportsVTable,

    /* boolean visitDevice (in string deviceID, in nsICacheDeviceInfo deviceInfo); */
    pub visitDevice: unsafe extern "C" fn (this: *const nsICacheVisitor, deviceID: *const libc::c_char, deviceInfo: *const nsICacheDeviceInfo, _retval: *mut bool) -> nsresult,

    /* boolean visitEntry (in string deviceID, in nsICacheEntryInfo entryInfo); */
    pub visitEntry: unsafe extern "C" fn (this: *const nsICacheVisitor, deviceID: *const libc::c_char, entryInfo: *const nsICacheEntryInfo, _retval: *mut bool) -> nsresult,

}


impl nsICacheVisitor {
    /* boolean visitDevice (in string deviceID, in nsICacheDeviceInfo deviceInfo); */
    #[inline]
    pub unsafe fn visitDevice(&self, deviceID: *const libc::c_char, deviceInfo: Option<&nsICacheDeviceInfo>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).visitDevice)(self as *const _, deviceID, deviceInfo.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean visitEntry (in string deviceID, in nsICacheEntryInfo entryInfo); */
    #[inline]
    pub unsafe fn visitEntry(&self, deviceID: *const libc::c_char, entryInfo: Option<&nsICacheEntryInfo>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).visitEntry)(self as *const _, deviceID, entryInfo.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsICacheDeviceInfo {
    vtable: *const nsICacheDeviceInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICacheDeviceInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x31d1c294, 0x1dd2, 0x11b2,
            [0xbe, 0x3a, 0xc7, 0x92, 0x30, 0xdc, 0xa2, 0x97])
    }
}

unsafe impl RefCounted for nsICacheDeviceInfo {
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
pub trait nsICacheDeviceInfoCoerce {
    fn coerce_from(v: &nsICacheDeviceInfo) -> &Self;
}

impl nsICacheDeviceInfoCoerce for nsICacheDeviceInfo {
    #[inline]
    fn coerce_from(v: &nsICacheDeviceInfo) -> &Self {
        v
    }
}

impl nsICacheDeviceInfo {
    #[inline]
    pub fn coerce<T: nsICacheDeviceInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICacheDeviceInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICacheDeviceInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheDeviceInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICacheDeviceInfoVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute string description; */
    pub get_description: unsafe extern "C" fn (this: *const nsICacheDeviceInfo, aDescription: *mut *const libc::c_char) -> nsresult,

    /* readonly attribute string usageReport; */
    pub get_usageReport: unsafe extern "C" fn (this: *const nsICacheDeviceInfo, aUsageReport: *mut *const libc::c_char) -> nsresult,

    /* readonly attribute unsigned long entryCount; */
    pub get_entryCount: unsafe extern "C" fn (this: *const nsICacheDeviceInfo, aEntryCount: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute unsigned long totalSize; */
    pub get_totalSize: unsafe extern "C" fn (this: *const nsICacheDeviceInfo, aTotalSize: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute unsigned long maximumSize; */
    pub get_maximumSize: unsafe extern "C" fn (this: *const nsICacheDeviceInfo, aMaximumSize: *mut libc::uint32_t) -> nsresult,

}


impl nsICacheDeviceInfo {
    /* readonly attribute string description; */
    #[inline]
    pub unsafe fn get_description(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_description)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute string usageReport; */
    #[inline]
    pub unsafe fn get_usageReport(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_usageReport)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long entryCount; */
    #[inline]
    pub unsafe fn get_entryCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_entryCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long totalSize; */
    #[inline]
    pub unsafe fn get_totalSize(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_totalSize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long maximumSize; */
    #[inline]
    pub unsafe fn get_maximumSize(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_maximumSize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsICacheEntryInfo {
    vtable: *const nsICacheEntryInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICacheEntryInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfab51c92, 0x95c3, 0x4468,
            [0xb3, 0x17, 0x7d, 0xe4, 0xd7, 0x58, 0x82, 0x54])
    }
}

unsafe impl RefCounted for nsICacheEntryInfo {
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
pub trait nsICacheEntryInfoCoerce {
    fn coerce_from(v: &nsICacheEntryInfo) -> &Self;
}

impl nsICacheEntryInfoCoerce for nsICacheEntryInfo {
    #[inline]
    fn coerce_from(v: &nsICacheEntryInfo) -> &Self {
        v
    }
}

impl nsICacheEntryInfo {
    #[inline]
    pub fn coerce<T: nsICacheEntryInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICacheEntryInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICacheEntryInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheEntryInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICacheEntryInfoVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute string clientID; */
    pub get_clientID: unsafe extern "C" fn (this: *const nsICacheEntryInfo, aClientID: *mut *const libc::c_char) -> nsresult,

    /* readonly attribute string deviceID; */
    pub get_deviceID: unsafe extern "C" fn (this: *const nsICacheEntryInfo, aDeviceID: *mut *const libc::c_char) -> nsresult,

    /* readonly attribute ACString key; */
    pub get_key: unsafe extern "C" fn (this: *const nsICacheEntryInfo, aKey: *mut nsACString) -> nsresult,

    /* readonly attribute long fetchCount; */
    pub get_fetchCount: unsafe extern "C" fn (this: *const nsICacheEntryInfo, aFetchCount: *mut libc::int32_t) -> nsresult,

    /* readonly attribute uint32_t lastFetched; */
    pub get_lastFetched: unsafe extern "C" fn (this: *const nsICacheEntryInfo, aLastFetched: *mut uint32_t) -> nsresult,

    /* readonly attribute uint32_t lastModified; */
    pub get_lastModified: unsafe extern "C" fn (this: *const nsICacheEntryInfo, aLastModified: *mut uint32_t) -> nsresult,

    /* readonly attribute uint32_t expirationTime; */
    pub get_expirationTime: unsafe extern "C" fn (this: *const nsICacheEntryInfo, aExpirationTime: *mut uint32_t) -> nsresult,

    /* readonly attribute unsigned long dataSize; */
    pub get_dataSize: unsafe extern "C" fn (this: *const nsICacheEntryInfo, aDataSize: *mut libc::uint32_t) -> nsresult,

    /* boolean isStreamBased (); */
    pub isStreamBased: unsafe extern "C" fn (this: *const nsICacheEntryInfo, _retval: *mut bool) -> nsresult,

}


impl nsICacheEntryInfo {
    /* readonly attribute string clientID; */
    #[inline]
    pub unsafe fn get_clientID(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_clientID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute string deviceID; */
    #[inline]
    pub unsafe fn get_deviceID(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_deviceID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString key; */
    #[inline]
    pub unsafe fn get_key(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_key)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long fetchCount; */
    #[inline]
    pub unsafe fn get_fetchCount(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_fetchCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute uint32_t lastFetched; */
    #[inline]
    pub unsafe fn get_lastFetched(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_lastFetched)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute uint32_t lastModified; */
    #[inline]
    pub unsafe fn get_lastModified(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_lastModified)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute uint32_t expirationTime; */
    #[inline]
    pub unsafe fn get_expirationTime(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_expirationTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long dataSize; */
    #[inline]
    pub unsafe fn get_dataSize(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_dataSize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isStreamBased (); */
    #[inline]
    pub unsafe fn isStreamBased(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isStreamBased)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


