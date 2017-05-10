//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILoginMetaInfo.idl
//


#[repr(C)]
pub struct nsILoginMetaInfo {
    vtable: *const nsILoginMetaInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsILoginMetaInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x20d8eb40, 0xc494, 0x497f,
            [0xb2, 0xa6, 0xaa, 0xa3, 0x2f, 0x80, 0x7e, 0xbd])
    }
}

unsafe impl RefCounted for nsILoginMetaInfo {
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
pub trait nsILoginMetaInfoCoerce {
    fn coerce_from(v: &nsILoginMetaInfo) -> &Self;
}

impl nsILoginMetaInfoCoerce for nsILoginMetaInfo {
    #[inline]
    fn coerce_from(v: &nsILoginMetaInfo) -> &Self {
        v
    }
}

impl nsILoginMetaInfo {
    #[inline]
    pub fn coerce<T: nsILoginMetaInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsILoginMetaInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsILoginMetaInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoginMetaInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsILoginMetaInfoVTable {
    pub __base: nsISupportsVTable,

    /* attribute AString guid; */
    pub get_guid: unsafe extern "C" fn (this: *const nsILoginMetaInfo, aGuid: *mut nsAString) -> nsresult,
    pub set_guid: unsafe extern "C" fn (this: *const nsILoginMetaInfo, aGuid: *const nsAString) -> nsresult,

    /* attribute unsigned long long timeCreated; */
    pub get_timeCreated: unsafe extern "C" fn (this: *const nsILoginMetaInfo, aTimeCreated: *mut libc::uint64_t) -> nsresult,
    pub set_timeCreated: unsafe extern "C" fn (this: *const nsILoginMetaInfo, aTimeCreated: libc::uint64_t) -> nsresult,

    /* attribute unsigned long long timeLastUsed; */
    pub get_timeLastUsed: unsafe extern "C" fn (this: *const nsILoginMetaInfo, aTimeLastUsed: *mut libc::uint64_t) -> nsresult,
    pub set_timeLastUsed: unsafe extern "C" fn (this: *const nsILoginMetaInfo, aTimeLastUsed: libc::uint64_t) -> nsresult,

    /* attribute unsigned long long timePasswordChanged; */
    pub get_timePasswordChanged: unsafe extern "C" fn (this: *const nsILoginMetaInfo, aTimePasswordChanged: *mut libc::uint64_t) -> nsresult,
    pub set_timePasswordChanged: unsafe extern "C" fn (this: *const nsILoginMetaInfo, aTimePasswordChanged: libc::uint64_t) -> nsresult,

    /* attribute unsigned long timesUsed; */
    pub get_timesUsed: unsafe extern "C" fn (this: *const nsILoginMetaInfo, aTimesUsed: *mut libc::uint32_t) -> nsresult,
    pub set_timesUsed: unsafe extern "C" fn (this: *const nsILoginMetaInfo, aTimesUsed: libc::uint32_t) -> nsresult,

}


impl nsILoginMetaInfo {
    /* attribute AString guid; */
    #[inline]
    pub unsafe fn get_guid(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_guid)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_guid(&self, aGuid: &[u16]) -> Result<(), nsresult> {
        let aGuid = nsString::from(aGuid);
        match ((*self.vtable).set_guid)(self as *const _, &*aGuid) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long long timeCreated; */
    #[inline]
    pub unsafe fn get_timeCreated(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_timeCreated)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_timeCreated(&self, aTimeCreated: libc::uint64_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_timeCreated)(self as *const _, aTimeCreated) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long long timeLastUsed; */
    #[inline]
    pub unsafe fn get_timeLastUsed(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_timeLastUsed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_timeLastUsed(&self, aTimeLastUsed: libc::uint64_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_timeLastUsed)(self as *const _, aTimeLastUsed) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long long timePasswordChanged; */
    #[inline]
    pub unsafe fn get_timePasswordChanged(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_timePasswordChanged)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_timePasswordChanged(&self, aTimePasswordChanged: libc::uint64_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_timePasswordChanged)(self as *const _, aTimePasswordChanged) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long timesUsed; */
    #[inline]
    pub unsafe fn get_timesUsed(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_timesUsed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_timesUsed(&self, aTimesUsed: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_timesUsed)(self as *const _, aTimesUsed) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


