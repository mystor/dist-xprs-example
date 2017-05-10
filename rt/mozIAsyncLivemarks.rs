//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIAsyncLivemarks.idl
//


#[repr(C)]
pub struct mozIAsyncLivemarks {
    vtable: *const mozIAsyncLivemarksVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIAsyncLivemarks {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x672387b7, 0xa75d, 0x4e8f,
            [0x9b, 0x49, 0x5c, 0x1d, 0xcb, 0xff, 0xf4, 0x6b])
    }
}

unsafe impl RefCounted for mozIAsyncLivemarks {
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
pub trait mozIAsyncLivemarksCoerce {
    fn coerce_from(v: &mozIAsyncLivemarks) -> &Self;
}

impl mozIAsyncLivemarksCoerce for mozIAsyncLivemarks {
    #[inline]
    fn coerce_from(v: &mozIAsyncLivemarks) -> &Self {
        v
    }
}

impl mozIAsyncLivemarks {
    #[inline]
    pub fn coerce<T: mozIAsyncLivemarksCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIAsyncLivemarks {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIAsyncLivemarksCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIAsyncLivemarks) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIAsyncLivemarksVTable {
    pub __base: nsISupportsVTable,

    /* jsval addLivemark (in jsval aLivemarkInfo); */
    /// Unable to call function as its signature contains a non-rust type
    pub addLivemark: *const ::libc::c_void,

    /* jsval removeLivemark (in jsval aLivemarkInfo); */
    /// Unable to call function as its signature contains a non-rust type
    pub removeLivemark: *const ::libc::c_void,

    /* jsval getLivemark (in jsval aLivemarkInfo); */
    /// Unable to call function as its signature contains a non-rust type
    pub getLivemark: *const ::libc::c_void,

    /* void reloadLivemarks ([optional] in boolean aForceUpdate); */
    pub reloadLivemarks: unsafe extern "C" fn (this: *const mozIAsyncLivemarks, aForceUpdate: bool) -> nsresult,

}


impl mozIAsyncLivemarks {
    /* jsval addLivemark (in jsval aLivemarkInfo); */


    /* jsval removeLivemark (in jsval aLivemarkInfo); */


    /* jsval getLivemark (in jsval aLivemarkInfo); */


    /* void reloadLivemarks ([optional] in boolean aForceUpdate); */
    #[inline]
    pub unsafe fn reloadLivemarks(&self, aForceUpdate: bool) -> Result<(), nsresult> {

        match ((*self.vtable).reloadLivemarks)(self as *const _, aForceUpdate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct mozILivemarkInfo {
    vtable: *const mozILivemarkInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozILivemarkInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3a3c5e8f, 0xec4a, 0x4086,
            [0xae, 0x0a, 0xd1, 0x64, 0x20, 0xd3, 0x0c, 0x9f])
    }
}

unsafe impl RefCounted for mozILivemarkInfo {
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
pub trait mozILivemarkInfoCoerce {
    fn coerce_from(v: &mozILivemarkInfo) -> &Self;
}

impl mozILivemarkInfoCoerce for mozILivemarkInfo {
    #[inline]
    fn coerce_from(v: &mozILivemarkInfo) -> &Self {
        v
    }
}

impl mozILivemarkInfo {
    #[inline]
    pub fn coerce<T: mozILivemarkInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozILivemarkInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozILivemarkInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &mozILivemarkInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozILivemarkInfoVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute long long id; */
    pub get_id: unsafe extern "C" fn (this: *const mozILivemarkInfo, aId: *mut libc::int64_t) -> nsresult,

    /* readonly attribute ACString guid; */
    pub get_guid: unsafe extern "C" fn (this: *const mozILivemarkInfo, aGuid: *mut nsACString) -> nsresult,

    /* readonly attribute AString title; */
    pub get_title: unsafe extern "C" fn (this: *const mozILivemarkInfo, aTitle: *mut nsAString) -> nsresult,

    /* readonly attribute long long parentId; */
    pub get_parentId: unsafe extern "C" fn (this: *const mozILivemarkInfo, aParentId: *mut libc::int64_t) -> nsresult,

    /* readonly attribute long long parentGuid; */
    pub get_parentGuid: unsafe extern "C" fn (this: *const mozILivemarkInfo, aParentGuid: *mut libc::int64_t) -> nsresult,

    /* readonly attribute long index; */
    pub get_index: unsafe extern "C" fn (this: *const mozILivemarkInfo, aIndex: *mut libc::int32_t) -> nsresult,

    /* readonly attribute PRTime dateAdded; */
    pub get_dateAdded: unsafe extern "C" fn (this: *const mozILivemarkInfo, aDateAdded: *mut PRTime) -> nsresult,

    /* readonly attribute PRTime lastModified; */
    pub get_lastModified: unsafe extern "C" fn (this: *const mozILivemarkInfo, aLastModified: *mut PRTime) -> nsresult,

    /* readonly attribute nsIURI feedURI; */
    pub get_feedURI: unsafe extern "C" fn (this: *const mozILivemarkInfo, aFeedURI: *mut *const nsIURI) -> nsresult,

    /* readonly attribute nsIURI siteURI; */
    pub get_siteURI: unsafe extern "C" fn (this: *const mozILivemarkInfo, aSiteURI: *mut *const nsIURI) -> nsresult,

}


impl mozILivemarkInfo {
    /* readonly attribute long long id; */
    #[inline]
    pub unsafe fn get_id(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
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

    /* readonly attribute AString title; */
    #[inline]
    pub unsafe fn get_title(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_title)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long long parentId; */
    #[inline]
    pub unsafe fn get_parentId(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_parentId)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long long parentGuid; */
    #[inline]
    pub unsafe fn get_parentGuid(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_parentGuid)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long index; */
    #[inline]
    pub unsafe fn get_index(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_index)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime dateAdded; */
    #[inline]
    pub unsafe fn get_dateAdded(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_dateAdded)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime lastModified; */
    #[inline]
    pub unsafe fn get_lastModified(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_lastModified)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIURI feedURI; */
    #[inline]
    pub unsafe fn get_feedURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_feedURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIURI siteURI; */
    #[inline]
    pub unsafe fn get_siteURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_siteURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


pub mod mozILivemark_consts {
    pub const STATUS_READY: i64 = 0;
    pub const STATUS_LOADING: i64 = 1;
    pub const STATUS_FAILED: i64 = 2;
}


#[repr(C)]
pub struct mozILivemark {
    vtable: *const mozILivemarkVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozILivemark {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9f6fdfae, 0xdb9a, 0x4bd8,
            [0xbd, 0xe1, 0x14, 0x87, 0x58, 0xcf, 0x1b, 0x18])
    }
}

unsafe impl RefCounted for mozILivemark {
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
pub trait mozILivemarkCoerce {
    fn coerce_from(v: &mozILivemark) -> &Self;
}

impl mozILivemarkCoerce for mozILivemark {
    #[inline]
    fn coerce_from(v: &mozILivemark) -> &Self {
        v
    }
}

impl mozILivemark {
    #[inline]
    pub fn coerce<T: mozILivemarkCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozILivemark {
    type Target = mozILivemarkInfo;
    #[inline]
    fn deref(&self) -> &mozILivemarkInfo {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: mozILivemarkInfoCoerce> mozILivemarkCoerce for T {
    #[inline]
    fn coerce_from(v: &mozILivemark) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozILivemarkVTable {
    pub __base: mozILivemarkInfoVTable,

    /* readonly attribute unsigned short status; */
    pub get_status: unsafe extern "C" fn (this: *const mozILivemark, aStatus: *mut libc::uint16_t) -> nsresult,

    /* void reload ([optional] in boolean aForceUpdate); */
    pub reload: unsafe extern "C" fn (this: *const mozILivemark, aForceUpdate: bool) -> nsresult,

    /* jsval getNodesForContainer (in jsval aContainerNode); */
    /// Unable to call function as its signature contains a non-rust type
    pub getNodesForContainer: *const ::libc::c_void,

    /* void registerForUpdates (in jsval aContainerNode, in nsINavHistoryResultObserver aResultObserver); */
    /// Unable to call function as its signature contains a non-rust type
    pub registerForUpdates: *const ::libc::c_void,

    /* void unregisterForUpdates (in jsval aContainerNode); */
    /// Unable to call function as its signature contains a non-rust type
    pub unregisterForUpdates: *const ::libc::c_void,

}


impl mozILivemark {
    /* readonly attribute unsigned short status; */
    #[inline]
    pub unsafe fn get_status(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_status)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void reload ([optional] in boolean aForceUpdate); */
    #[inline]
    pub unsafe fn reload(&self, aForceUpdate: bool) -> Result<(), nsresult> {

        match ((*self.vtable).reload)(self as *const _, aForceUpdate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* jsval getNodesForContainer (in jsval aContainerNode); */


    /* void registerForUpdates (in jsval aContainerNode, in nsINavHistoryResultObserver aResultObserver); */


    /* void unregisterForUpdates (in jsval aContainerNode); */


}


