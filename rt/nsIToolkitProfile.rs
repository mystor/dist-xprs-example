//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIToolkitProfile.idl
//


#[repr(C)]
pub struct nsIProfileLock {
    vtable: *const nsIProfileLockVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIProfileLock {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7c58c703, 0xd245, 0x4864,
            [0x8d, 0x75, 0x96, 0x48, 0xca, 0x4a, 0x61, 0x39])
    }
}

unsafe impl RefCounted for nsIProfileLock {
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
pub trait nsIProfileLockCoerce {
    fn coerce_from(v: &nsIProfileLock) -> &Self;
}

impl nsIProfileLockCoerce for nsIProfileLock {
    #[inline]
    fn coerce_from(v: &nsIProfileLock) -> &Self {
        v
    }
}

impl nsIProfileLock {
    #[inline]
    pub fn coerce<T: nsIProfileLockCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIProfileLock {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIProfileLockCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProfileLock) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIProfileLockVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIFile directory; */
    pub get_directory: unsafe extern "C" fn (this: *const nsIProfileLock, aDirectory: *mut *const nsIFile) -> nsresult,

    /* readonly attribute nsIFile localDirectory; */
    pub get_localDirectory: unsafe extern "C" fn (this: *const nsIProfileLock, aLocalDirectory: *mut *const nsIFile) -> nsresult,

    /* readonly attribute PRTime replacedLockTime; */
    pub get_replacedLockTime: unsafe extern "C" fn (this: *const nsIProfileLock, aReplacedLockTime: *mut PRTime) -> nsresult,

    /* void unlock (); */
    pub unlock: unsafe extern "C" fn (this: *const nsIProfileLock) -> nsresult,

}


impl nsIProfileLock {
    /* readonly attribute nsIFile directory; */
    #[inline]
    pub unsafe fn get_directory(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_directory)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIFile localDirectory; */
    #[inline]
    pub unsafe fn get_localDirectory(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_localDirectory)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute PRTime replacedLockTime; */
    #[inline]
    pub unsafe fn get_replacedLockTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_replacedLockTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void unlock (); */
    #[inline]
    pub unsafe fn unlock(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).unlock)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIToolkitProfile {
    vtable: *const nsIToolkitProfileVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIToolkitProfile {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7422b090, 0x4a86, 0x4407,
            [0x97, 0x2e, 0x75, 0x46, 0x8a, 0x62, 0x53, 0x88])
    }
}

unsafe impl RefCounted for nsIToolkitProfile {
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
pub trait nsIToolkitProfileCoerce {
    fn coerce_from(v: &nsIToolkitProfile) -> &Self;
}

impl nsIToolkitProfileCoerce for nsIToolkitProfile {
    #[inline]
    fn coerce_from(v: &nsIToolkitProfile) -> &Self {
        v
    }
}

impl nsIToolkitProfile {
    #[inline]
    pub fn coerce<T: nsIToolkitProfileCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIToolkitProfile {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIToolkitProfileCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIToolkitProfile) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIToolkitProfileVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIFile rootDir; */
    pub get_rootDir: unsafe extern "C" fn (this: *const nsIToolkitProfile, aRootDir: *mut *const nsIFile) -> nsresult,

    /* readonly attribute nsIFile localDir; */
    pub get_localDir: unsafe extern "C" fn (this: *const nsIToolkitProfile, aLocalDir: *mut *const nsIFile) -> nsresult,

    /* attribute AUTF8String name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIToolkitProfile, aName: *mut nsACString) -> nsresult,
    pub set_name: unsafe extern "C" fn (this: *const nsIToolkitProfile, aName: *const nsACString) -> nsresult,

    /* void remove (in boolean removeFiles); */
    pub remove: unsafe extern "C" fn (this: *const nsIToolkitProfile, removeFiles: bool) -> nsresult,

    /* nsIProfileLock lock (out nsIProfileUnlocker aUnlocker); */
    pub lock: unsafe extern "C" fn (this: *const nsIToolkitProfile, aUnlocker: *mut *const nsIProfileUnlocker, _retval: *mut *const nsIProfileLock) -> nsresult,

}


impl nsIToolkitProfile {
    /* readonly attribute nsIFile rootDir; */
    #[inline]
    pub unsafe fn get_rootDir(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_rootDir)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIFile localDir; */
    #[inline]
    pub unsafe fn get_localDir(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_localDir)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute AUTF8String name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_name(&self, aName: &[u8]) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        match ((*self.vtable).set_name)(self as *const _, &*aName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void remove (in boolean removeFiles); */
    #[inline]
    pub unsafe fn remove(&self, removeFiles: bool) -> Result<(), nsresult> {

        match ((*self.vtable).remove)(self as *const _, removeFiles) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIProfileLock lock (out nsIProfileUnlocker aUnlocker); */
    #[inline]
    pub unsafe fn lock(&self, ) -> Result<(Option<RefPtr<nsIProfileUnlocker>>, Option<RefPtr<nsIProfileLock>>), nsresult> {
        let mut aUnlocker = GetterAddrefs::new();
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).lock)(self as *const _, aUnlocker.ptr(), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aUnlocker.refptr(), _retval.refptr()))
    }

}


