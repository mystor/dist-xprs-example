//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDownload.idl
//


#[repr(C)]
pub struct nsIDownload {
    vtable: *const nsIDownloadVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDownload {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2258f465, 0x656e, 0x4566,
            [0x87, 0xcb, 0xf7, 0x91, 0xdb, 0xaf, 0x03, 0x22])
    }
}

unsafe impl RefCounted for nsIDownload {
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
pub trait nsIDownloadCoerce {
    fn coerce_from(v: &nsIDownload) -> &Self;
}

impl nsIDownloadCoerce for nsIDownload {
    #[inline]
    fn coerce_from(v: &nsIDownload) -> &Self {
        v
    }
}

impl nsIDownload {
    #[inline]
    pub fn coerce<T: nsIDownloadCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDownload {
    type Target = nsITransfer;
    #[inline]
    fn deref(&self) -> &nsITransfer {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsITransferCoerce> nsIDownloadCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDownload) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDownloadVTable {
    pub __base: nsITransferVTable,

    /* readonly attribute nsIFile targetFile; */
    pub get_targetFile: unsafe extern "C" fn (this: *const nsIDownload, aTargetFile: *mut *const nsIFile) -> nsresult,

    /* readonly attribute long percentComplete; */
    pub get_percentComplete: unsafe extern "C" fn (this: *const nsIDownload, aPercentComplete: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long long amountTransferred; */
    pub get_amountTransferred: unsafe extern "C" fn (this: *const nsIDownload, aAmountTransferred: *mut libc::int64_t) -> nsresult,

    /* readonly attribute long long size; */
    pub get_size: unsafe extern "C" fn (this: *const nsIDownload, aSize: *mut libc::int64_t) -> nsresult,

    /* readonly attribute nsIURI source; */
    pub get_source: unsafe extern "C" fn (this: *const nsIDownload, aSource: *mut *const nsIURI) -> nsresult,

    /* readonly attribute nsIURI target; */
    pub get_target: unsafe extern "C" fn (this: *const nsIDownload, aTarget: *mut *const nsIURI) -> nsresult,

    /* readonly attribute nsICancelable cancelable; */
    pub get_cancelable: unsafe extern "C" fn (this: *const nsIDownload, aCancelable: *mut *const nsICancelable) -> nsresult,

    /* readonly attribute AString displayName; */
    pub get_displayName: unsafe extern "C" fn (this: *const nsIDownload, aDisplayName: *mut nsAString) -> nsresult,

    /* readonly attribute long long startTime; */
    pub get_startTime: unsafe extern "C" fn (this: *const nsIDownload, aStartTime: *mut libc::int64_t) -> nsresult,

    /* readonly attribute double speed; */
    pub get_speed: unsafe extern "C" fn (this: *const nsIDownload, aSpeed: *mut libc::c_double) -> nsresult,

    /* readonly attribute nsIMIMEInfo MIMEInfo; */
    pub get_MIMEInfo: unsafe extern "C" fn (this: *const nsIDownload, aMIMEInfo: *mut *const nsIMIMEInfo) -> nsresult,

    /* readonly attribute unsigned long id; */
    pub get_id: unsafe extern "C" fn (this: *const nsIDownload, aId: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute ACString guid; */
    pub get_guid: unsafe extern "C" fn (this: *const nsIDownload, aGuid: *mut nsACString) -> nsresult,

    /* readonly attribute short state; */
    pub get_state: unsafe extern "C" fn (this: *const nsIDownload, aState: *mut libc::int16_t) -> nsresult,

    /* readonly attribute nsIURI referrer; */
    pub get_referrer: unsafe extern "C" fn (this: *const nsIDownload, aReferrer: *mut *const nsIURI) -> nsresult,

    /* readonly attribute boolean resumable; */
    pub get_resumable: unsafe extern "C" fn (this: *const nsIDownload, aResumable: *mut bool) -> nsresult,

    /* readonly attribute boolean isPrivate; */
    pub get_isPrivate: unsafe extern "C" fn (this: *const nsIDownload, aIsPrivate: *mut bool) -> nsresult,

    /* void cancel (); */
    pub cancel: unsafe extern "C" fn (this: *const nsIDownload) -> nsresult,

    /* void pause (); */
    pub pause: unsafe extern "C" fn (this: *const nsIDownload) -> nsresult,

    /* void resume (); */
    pub resume: unsafe extern "C" fn (this: *const nsIDownload) -> nsresult,

    /* void remove (); */
    pub remove: unsafe extern "C" fn (this: *const nsIDownload) -> nsresult,

    /* void retry (); */
    pub retry: unsafe extern "C" fn (this: *const nsIDownload) -> nsresult,

}


impl nsIDownload {
    /* readonly attribute nsIFile targetFile; */
    #[inline]
    pub unsafe fn get_targetFile(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_targetFile)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long percentComplete; */
    #[inline]
    pub unsafe fn get_percentComplete(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_percentComplete)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long long amountTransferred; */
    #[inline]
    pub unsafe fn get_amountTransferred(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_amountTransferred)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long long size; */
    #[inline]
    pub unsafe fn get_size(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_size)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIURI source; */
    #[inline]
    pub unsafe fn get_source(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_source)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIURI target; */
    #[inline]
    pub unsafe fn get_target(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_target)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsICancelable cancelable; */
    #[inline]
    pub unsafe fn get_cancelable(&self, ) -> Result<Option<RefPtr<nsICancelable>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_cancelable)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute AString displayName; */
    #[inline]
    pub unsafe fn get_displayName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_displayName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long long startTime; */
    #[inline]
    pub unsafe fn get_startTime(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_startTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute double speed; */
    #[inline]
    pub unsafe fn get_speed(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_speed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIMIMEInfo MIMEInfo; */
    #[inline]
    pub unsafe fn get_MIMEInfo(&self, ) -> Result<Option<RefPtr<nsIMIMEInfo>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_MIMEInfo)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute unsigned long id; */
    #[inline]
    pub unsafe fn get_id(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_id)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString guid; */
    #[inline]
    pub unsafe fn get_guid(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_guid)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute short state; */
    #[inline]
    pub unsafe fn get_state(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_state)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIURI referrer; */
    #[inline]
    pub unsafe fn get_referrer(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_referrer)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute boolean resumable; */
    #[inline]
    pub unsafe fn get_resumable(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_resumable)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isPrivate; */
    #[inline]
    pub unsafe fn get_isPrivate(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isPrivate)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void cancel (); */
    #[inline]
    pub unsafe fn cancel(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).cancel)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void pause (); */
    #[inline]
    pub unsafe fn pause(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).pause)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void resume (); */
    #[inline]
    pub unsafe fn resume(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).resume)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void remove (); */
    #[inline]
    pub unsafe fn remove(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).remove)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void retry (); */
    #[inline]
    pub unsafe fn retry(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).retry)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


