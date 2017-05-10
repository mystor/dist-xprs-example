//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICachingChannel.idl
//


pub mod nsICachingChannel_consts {
    pub const LOAD_NO_NETWORK_IO: i64 = 67108864;
    pub const LOAD_CHECK_OFFLINE_CACHE: i64 = 134217728;
    pub const LOAD_BYPASS_LOCAL_CACHE: i64 = 268435456;
    pub const LOAD_BYPASS_LOCAL_CACHE_IF_BUSY: i64 = 536870912;
    pub const LOAD_ONLY_FROM_CACHE: i64 = 1073741824;
    pub const LOAD_ONLY_IF_MODIFIED: i64 = 2147483648;
}


#[repr(C)]
pub struct nsICachingChannel {
    vtable: *const nsICachingChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICachingChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xdd1d6122, 0x5ecf, 0x4fe4,
            [0x8f, 0x0f, 0x99, 0x5e, 0x7a, 0xb3, 0x12, 0x1a])
    }
}

unsafe impl RefCounted for nsICachingChannel {
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
pub trait nsICachingChannelCoerce {
    fn coerce_from(v: &nsICachingChannel) -> &Self;
}

impl nsICachingChannelCoerce for nsICachingChannel {
    #[inline]
    fn coerce_from(v: &nsICachingChannel) -> &Self {
        v
    }
}

impl nsICachingChannel {
    #[inline]
    pub fn coerce<T: nsICachingChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICachingChannel {
    type Target = nsICacheInfoChannel;
    #[inline]
    fn deref(&self) -> &nsICacheInfoChannel {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsICacheInfoChannelCoerce> nsICachingChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICachingChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICachingChannelVTable {
    pub __base: nsICacheInfoChannelVTable,

    /* attribute nsISupports cacheToken; */
    pub get_cacheToken: unsafe extern "C" fn (this: *const nsICachingChannel, aCacheToken: *mut *const nsISupports) -> nsresult,
    pub set_cacheToken: unsafe extern "C" fn (this: *const nsICachingChannel, aCacheToken: *const nsISupports) -> nsresult,

    /* attribute nsISupports offlineCacheToken; */
    pub get_offlineCacheToken: unsafe extern "C" fn (this: *const nsICachingChannel, aOfflineCacheToken: *mut *const nsISupports) -> nsresult,
    pub set_offlineCacheToken: unsafe extern "C" fn (this: *const nsICachingChannel, aOfflineCacheToken: *const nsISupports) -> nsresult,

    /* attribute boolean cacheOnlyMetadata; */
    pub get_cacheOnlyMetadata: unsafe extern "C" fn (this: *const nsICachingChannel, aCacheOnlyMetadata: *mut bool) -> nsresult,
    pub set_cacheOnlyMetadata: unsafe extern "C" fn (this: *const nsICachingChannel, aCacheOnlyMetadata: bool) -> nsresult,

    /* attribute boolean pin; */
    pub get_pin: unsafe extern "C" fn (this: *const nsICachingChannel, aPin: *mut bool) -> nsresult,
    pub set_pin: unsafe extern "C" fn (this: *const nsICachingChannel, aPin: bool) -> nsresult,

    /* void forceCacheEntryValidFor (in unsigned long aSecondsToTheFuture); */
    pub forceCacheEntryValidFor: unsafe extern "C" fn (this: *const nsICachingChannel, aSecondsToTheFuture: libc::uint32_t) -> nsresult,

}


impl nsICachingChannel {
    /* attribute nsISupports cacheToken; */
    #[inline]
    pub unsafe fn get_cacheToken(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_cacheToken)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_cacheToken(&self, aCacheToken: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).set_cacheToken)(self as *const _, aCacheToken.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsISupports offlineCacheToken; */
    #[inline]
    pub unsafe fn get_offlineCacheToken(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_offlineCacheToken)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_offlineCacheToken(&self, aOfflineCacheToken: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).set_offlineCacheToken)(self as *const _, aOfflineCacheToken.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean cacheOnlyMetadata; */
    #[inline]
    pub unsafe fn get_cacheOnlyMetadata(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_cacheOnlyMetadata)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_cacheOnlyMetadata(&self, aCacheOnlyMetadata: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_cacheOnlyMetadata)(self as *const _, aCacheOnlyMetadata) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean pin; */
    #[inline]
    pub unsafe fn get_pin(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_pin)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_pin(&self, aPin: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_pin)(self as *const _, aPin) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void forceCacheEntryValidFor (in unsigned long aSecondsToTheFuture); */
    #[inline]
    pub unsafe fn forceCacheEntryValidFor(&self, aSecondsToTheFuture: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).forceCacheEntryValidFor)(self as *const _, aSecondsToTheFuture) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


