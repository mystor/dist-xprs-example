//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIApplicationCacheContainer.idl
//


#[repr(C)]
pub struct nsIApplicationCacheContainer {
    vtable: *const nsIApplicationCacheContainerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIApplicationCacheContainer {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbbb80700, 0x1f7f, 0x4258,
            [0xaf, 0xf4, 0x17, 0x43, 0xcc, 0x5a, 0x7d, 0x23])
    }
}

unsafe impl RefCounted for nsIApplicationCacheContainer {
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
pub trait nsIApplicationCacheContainerCoerce {
    fn coerce_from(v: &nsIApplicationCacheContainer) -> &Self;
}

impl nsIApplicationCacheContainerCoerce for nsIApplicationCacheContainer {
    #[inline]
    fn coerce_from(v: &nsIApplicationCacheContainer) -> &Self {
        v
    }
}

impl nsIApplicationCacheContainer {
    #[inline]
    pub fn coerce<T: nsIApplicationCacheContainerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIApplicationCacheContainer {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIApplicationCacheContainerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIApplicationCacheContainer) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIApplicationCacheContainerVTable {
    pub __base: nsISupportsVTable,

    /* attribute nsIApplicationCache applicationCache; */
    pub get_applicationCache: unsafe extern "C" fn (this: *const nsIApplicationCacheContainer, aApplicationCache: *mut *const nsIApplicationCache) -> nsresult,
    pub set_applicationCache: unsafe extern "C" fn (this: *const nsIApplicationCacheContainer, aApplicationCache: *const nsIApplicationCache) -> nsresult,

}


impl nsIApplicationCacheContainer {
    /* attribute nsIApplicationCache applicationCache; */
    #[inline]
    pub unsafe fn get_applicationCache(&self, ) -> Result<Option<RefPtr<nsIApplicationCache>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_applicationCache)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_applicationCache(&self, aApplicationCache: Option<&nsIApplicationCache>) -> Result<(), nsresult> {

        match ((*self.vtable).set_applicationCache)(self as *const _, aApplicationCache.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


