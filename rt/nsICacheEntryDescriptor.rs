//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICacheEntryDescriptor.idl
//


#[repr(C)]
pub struct nsICacheEntryDescriptor {
    vtable: *const nsICacheEntryDescriptorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICacheEntryDescriptor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x90b17d31, 0x46aa, 0x4fb1,
            [0xa2, 0x06, 0x47, 0x3c, 0x96, 0x6c, 0xbc, 0x18])
    }
}

unsafe impl RefCounted for nsICacheEntryDescriptor {
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
pub trait nsICacheEntryDescriptorCoerce {
    fn coerce_from(v: &nsICacheEntryDescriptor) -> &Self;
}

impl nsICacheEntryDescriptorCoerce for nsICacheEntryDescriptor {
    #[inline]
    fn coerce_from(v: &nsICacheEntryDescriptor) -> &Self {
        v
    }
}

impl nsICacheEntryDescriptor {
    #[inline]
    pub fn coerce<T: nsICacheEntryDescriptorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICacheEntryDescriptor {
    type Target = nsICacheEntryInfo;
    #[inline]
    fn deref(&self) -> &nsICacheEntryInfo {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsICacheEntryInfoCoerce> nsICacheEntryDescriptorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheEntryDescriptor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICacheEntryDescriptorVTable {
    pub __base: nsICacheEntryInfoVTable,

    /* void setExpirationTime (in uint32_t expirationTime); */
    pub setExpirationTime: unsafe extern "C" fn (this: *const nsICacheEntryDescriptor, expirationTime: uint32_t) -> nsresult,

    /* void setDataSize (in unsigned long size); */
    pub setDataSize: unsafe extern "C" fn (this: *const nsICacheEntryDescriptor, size: libc::uint32_t) -> nsresult,

    /* nsIInputStream openInputStream (in unsigned long offset); */
    pub openInputStream: unsafe extern "C" fn (this: *const nsICacheEntryDescriptor, offset: libc::uint32_t, _retval: *mut *const nsIInputStream) -> nsresult,

    /* nsIOutputStream openOutputStream (in unsigned long offset); */
    pub openOutputStream: unsafe extern "C" fn (this: *const nsICacheEntryDescriptor, offset: libc::uint32_t, _retval: *mut *const nsIOutputStream) -> nsresult,

    /* attribute nsISupports cacheElement; */
    pub get_cacheElement: unsafe extern "C" fn (this: *const nsICacheEntryDescriptor, aCacheElement: *mut *const nsISupports) -> nsresult,
    pub set_cacheElement: unsafe extern "C" fn (this: *const nsICacheEntryDescriptor, aCacheElement: *const nsISupports) -> nsresult,

    /* attribute int64_t predictedDataSize; */
    pub get_predictedDataSize: unsafe extern "C" fn (this: *const nsICacheEntryDescriptor, aPredictedDataSize: *mut int64_t) -> nsresult,
    pub set_predictedDataSize: unsafe extern "C" fn (this: *const nsICacheEntryDescriptor, aPredictedDataSize: int64_t) -> nsresult,

    /* readonly attribute nsCacheAccessMode accessGranted; */
    pub get_accessGranted: unsafe extern "C" fn (this: *const nsICacheEntryDescriptor, aAccessGranted: *mut nsCacheAccessMode) -> nsresult,

    /* attribute nsCacheStoragePolicy storagePolicy; */
    pub get_storagePolicy: unsafe extern "C" fn (this: *const nsICacheEntryDescriptor, aStoragePolicy: *mut nsCacheStoragePolicy) -> nsresult,
    pub set_storagePolicy: unsafe extern "C" fn (this: *const nsICacheEntryDescriptor, aStoragePolicy: nsCacheStoragePolicy) -> nsresult,

    /* readonly attribute nsIFile file; */
    pub get_file: unsafe extern "C" fn (this: *const nsICacheEntryDescriptor, aFile: *mut *const nsIFile) -> nsresult,

    /* attribute nsISupports securityInfo; */
    pub get_securityInfo: unsafe extern "C" fn (this: *const nsICacheEntryDescriptor, aSecurityInfo: *mut *const nsISupports) -> nsresult,
    pub set_securityInfo: unsafe extern "C" fn (this: *const nsICacheEntryDescriptor, aSecurityInfo: *const nsISupports) -> nsresult,

    /* readonly attribute unsigned long storageDataSize; */
    pub get_storageDataSize: unsafe extern "C" fn (this: *const nsICacheEntryDescriptor, aStorageDataSize: *mut libc::uint32_t) -> nsresult,

    /* void doom (); */
    pub doom: unsafe extern "C" fn (this: *const nsICacheEntryDescriptor) -> nsresult,

    /* void doomAndFailPendingRequests (in nsresult status); */
    pub doomAndFailPendingRequests: unsafe extern "C" fn (this: *const nsICacheEntryDescriptor, status: nsresult) -> nsresult,

    /* void asyncDoom (in nsICacheListener listener); */
    pub asyncDoom: unsafe extern "C" fn (this: *const nsICacheEntryDescriptor, listener: *const nsICacheListener) -> nsresult,

    /* void markValid (); */
    pub markValid: unsafe extern "C" fn (this: *const nsICacheEntryDescriptor) -> nsresult,

    /* void close (); */
    pub close: unsafe extern "C" fn (this: *const nsICacheEntryDescriptor) -> nsresult,

    /* string getMetaDataElement (in string key); */
    pub getMetaDataElement: unsafe extern "C" fn (this: *const nsICacheEntryDescriptor, key: *const libc::c_char, _retval: *mut *const libc::c_char) -> nsresult,

    /* void setMetaDataElement (in string key, in string value); */
    pub setMetaDataElement: unsafe extern "C" fn (this: *const nsICacheEntryDescriptor, key: *const libc::c_char, value: *const libc::c_char) -> nsresult,

    /* void visitMetaData (in nsICacheMetaDataVisitor visitor); */
    pub visitMetaData: unsafe extern "C" fn (this: *const nsICacheEntryDescriptor, visitor: *const nsICacheMetaDataVisitor) -> nsresult,

}


impl nsICacheEntryDescriptor {
    /* void setExpirationTime (in uint32_t expirationTime); */
    #[inline]
    pub unsafe fn setExpirationTime(&self, expirationTime: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setExpirationTime)(self as *const _, expirationTime) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setDataSize (in unsigned long size); */
    #[inline]
    pub unsafe fn setDataSize(&self, size: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setDataSize)(self as *const _, size) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIInputStream openInputStream (in unsigned long offset); */
    #[inline]
    pub unsafe fn openInputStream(&self, offset: libc::uint32_t) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).openInputStream)(self as *const _, offset, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIOutputStream openOutputStream (in unsigned long offset); */
    #[inline]
    pub unsafe fn openOutputStream(&self, offset: libc::uint32_t) -> Result<Option<RefPtr<nsIOutputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).openOutputStream)(self as *const _, offset, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute nsISupports cacheElement; */
    #[inline]
    pub unsafe fn get_cacheElement(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_cacheElement)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_cacheElement(&self, aCacheElement: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).set_cacheElement)(self as *const _, aCacheElement.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute int64_t predictedDataSize; */
    #[inline]
    pub unsafe fn get_predictedDataSize(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_predictedDataSize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_predictedDataSize(&self, aPredictedDataSize: int64_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_predictedDataSize)(self as *const _, aPredictedDataSize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsCacheAccessMode accessGranted; */
    #[inline]
    pub unsafe fn get_accessGranted(&self, ) -> Result<nsCacheAccessMode, nsresult> {
        let mut _retval: nsCacheAccessMode = ::std::mem::zeroed();
        match ((*self.vtable).get_accessGranted)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute nsCacheStoragePolicy storagePolicy; */
    #[inline]
    pub unsafe fn get_storagePolicy(&self, ) -> Result<nsCacheStoragePolicy, nsresult> {
        let mut _retval: nsCacheStoragePolicy = ::std::mem::zeroed();
        match ((*self.vtable).get_storagePolicy)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_storagePolicy(&self, aStoragePolicy: nsCacheStoragePolicy) -> Result<(), nsresult> {

        match ((*self.vtable).set_storagePolicy)(self as *const _, aStoragePolicy) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIFile file; */
    #[inline]
    pub unsafe fn get_file(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_file)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute nsISupports securityInfo; */
    #[inline]
    pub unsafe fn get_securityInfo(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_securityInfo)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_securityInfo(&self, aSecurityInfo: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).set_securityInfo)(self as *const _, aSecurityInfo.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute unsigned long storageDataSize; */
    #[inline]
    pub unsafe fn get_storageDataSize(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_storageDataSize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void doom (); */
    #[inline]
    pub unsafe fn doom(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).doom)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void doomAndFailPendingRequests (in nsresult status); */
    #[inline]
    pub unsafe fn doomAndFailPendingRequests(&self, status: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).doomAndFailPendingRequests)(self as *const _, status) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void asyncDoom (in nsICacheListener listener); */
    #[inline]
    pub unsafe fn asyncDoom(&self, listener: Option<&nsICacheListener>) -> Result<(), nsresult> {

        match ((*self.vtable).asyncDoom)(self as *const _, listener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void markValid (); */
    #[inline]
    pub unsafe fn markValid(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).markValid)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void close (); */
    #[inline]
    pub unsafe fn close(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).close)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* string getMetaDataElement (in string key); */
    #[inline]
    pub unsafe fn getMetaDataElement(&self, key: *const libc::c_char) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).getMetaDataElement)(self as *const _, key, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setMetaDataElement (in string key, in string value); */
    #[inline]
    pub unsafe fn setMetaDataElement(&self, key: *const libc::c_char, value: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).setMetaDataElement)(self as *const _, key, value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void visitMetaData (in nsICacheMetaDataVisitor visitor); */
    #[inline]
    pub unsafe fn visitMetaData(&self, visitor: Option<&nsICacheMetaDataVisitor>) -> Result<(), nsresult> {

        match ((*self.vtable).visitMetaData)(self as *const _, visitor.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsICacheMetaDataVisitor {
    vtable: *const nsICacheMetaDataVisitorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICacheMetaDataVisitor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x22f9a49c, 0x3cf8, 0x4c23,
            [0x80, 0x06, 0x54, 0xef, 0xb1, 0x1a, 0xc5, 0x62])
    }
}

unsafe impl RefCounted for nsICacheMetaDataVisitor {
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
pub trait nsICacheMetaDataVisitorCoerce {
    fn coerce_from(v: &nsICacheMetaDataVisitor) -> &Self;
}

impl nsICacheMetaDataVisitorCoerce for nsICacheMetaDataVisitor {
    #[inline]
    fn coerce_from(v: &nsICacheMetaDataVisitor) -> &Self {
        v
    }
}

impl nsICacheMetaDataVisitor {
    #[inline]
    pub fn coerce<T: nsICacheMetaDataVisitorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICacheMetaDataVisitor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICacheMetaDataVisitorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheMetaDataVisitor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICacheMetaDataVisitorVTable {
    pub __base: nsISupportsVTable,

    /* boolean visitMetaDataElement (in string key, in string value); */
    pub visitMetaDataElement: unsafe extern "C" fn (this: *const nsICacheMetaDataVisitor, key: *const libc::c_char, value: *const libc::c_char, _retval: *mut bool) -> nsresult,

}


impl nsICacheMetaDataVisitor {
    /* boolean visitMetaDataElement (in string key, in string value); */
    #[inline]
    pub unsafe fn visitMetaDataElement(&self, key: *const libc::c_char, value: *const libc::c_char) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).visitMetaDataElement)(self as *const _, key, value, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


