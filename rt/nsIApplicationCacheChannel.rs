//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIApplicationCacheChannel.idl
//


#[repr(C)]
pub struct nsIApplicationCacheChannel {
    vtable: *const nsIApplicationCacheChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIApplicationCacheChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6fa816b1, 0x6d5f, 0x4380,
            [0x97, 0x04, 0x05, 0x4d, 0x09, 0x08, 0xcf, 0xa3])
    }
}

unsafe impl RefCounted for nsIApplicationCacheChannel {
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
pub trait nsIApplicationCacheChannelCoerce {
    fn coerce_from(v: &nsIApplicationCacheChannel) -> &Self;
}

impl nsIApplicationCacheChannelCoerce for nsIApplicationCacheChannel {
    #[inline]
    fn coerce_from(v: &nsIApplicationCacheChannel) -> &Self {
        v
    }
}

impl nsIApplicationCacheChannel {
    #[inline]
    pub fn coerce<T: nsIApplicationCacheChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIApplicationCacheChannel {
    type Target = nsIApplicationCacheContainer;
    #[inline]
    fn deref(&self) -> &nsIApplicationCacheContainer {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIApplicationCacheContainerCoerce> nsIApplicationCacheChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIApplicationCacheChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIApplicationCacheChannelVTable {
    pub __base: nsIApplicationCacheContainerVTable,

    /* readonly attribute boolean loadedFromApplicationCache; */
    pub get_loadedFromApplicationCache: unsafe extern "C" fn (this: *const nsIApplicationCacheChannel, aLoadedFromApplicationCache: *mut bool) -> nsresult,

    /* attribute boolean inheritApplicationCache; */
    pub get_inheritApplicationCache: unsafe extern "C" fn (this: *const nsIApplicationCacheChannel, aInheritApplicationCache: *mut bool) -> nsresult,
    pub set_inheritApplicationCache: unsafe extern "C" fn (this: *const nsIApplicationCacheChannel, aInheritApplicationCache: bool) -> nsresult,

    /* attribute boolean chooseApplicationCache; */
    pub get_chooseApplicationCache: unsafe extern "C" fn (this: *const nsIApplicationCacheChannel, aChooseApplicationCache: *mut bool) -> nsresult,
    pub set_chooseApplicationCache: unsafe extern "C" fn (this: *const nsIApplicationCacheChannel, aChooseApplicationCache: bool) -> nsresult,

    /* void markOfflineCacheEntryAsForeign (); */
    pub markOfflineCacheEntryAsForeign: unsafe extern "C" fn (this: *const nsIApplicationCacheChannel) -> nsresult,

    /* attribute nsIApplicationCache applicationCacheForWrite; */
    pub get_applicationCacheForWrite: unsafe extern "C" fn (this: *const nsIApplicationCacheChannel, aApplicationCacheForWrite: *mut *const nsIApplicationCache) -> nsresult,
    pub set_applicationCacheForWrite: unsafe extern "C" fn (this: *const nsIApplicationCacheChannel, aApplicationCacheForWrite: *const nsIApplicationCache) -> nsresult,

}


impl nsIApplicationCacheChannel {
    /* readonly attribute boolean loadedFromApplicationCache; */
    #[inline]
    pub unsafe fn get_loadedFromApplicationCache(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_loadedFromApplicationCache)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute boolean inheritApplicationCache; */
    #[inline]
    pub unsafe fn get_inheritApplicationCache(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_inheritApplicationCache)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_inheritApplicationCache(&self, aInheritApplicationCache: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_inheritApplicationCache)(self as *const _, aInheritApplicationCache) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean chooseApplicationCache; */
    #[inline]
    pub unsafe fn get_chooseApplicationCache(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_chooseApplicationCache)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_chooseApplicationCache(&self, aChooseApplicationCache: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_chooseApplicationCache)(self as *const _, aChooseApplicationCache) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void markOfflineCacheEntryAsForeign (); */
    #[inline]
    pub unsafe fn markOfflineCacheEntryAsForeign(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).markOfflineCacheEntryAsForeign)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIApplicationCache applicationCacheForWrite; */
    #[inline]
    pub unsafe fn get_applicationCacheForWrite(&self, ) -> Result<Option<RefPtr<nsIApplicationCache>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_applicationCacheForWrite)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_applicationCacheForWrite(&self, aApplicationCacheForWrite: Option<&nsIApplicationCache>) -> Result<(), nsresult> {

        match ((*self.vtable).set_applicationCacheForWrite)(self as *const _, aApplicationCacheForWrite.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


