//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIGeolocationProvider.idl
//


#[repr(C)]
pub struct nsIGeolocationUpdate {
    vtable: *const nsIGeolocationUpdateVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIGeolocationUpdate {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x643dc5e9, 0xb911, 0x4b2c,
            [0x8d, 0x44, 0x60, 0x31, 0x62, 0x69, 0x6b, 0xaf])
    }
}

unsafe impl RefCounted for nsIGeolocationUpdate {
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
pub trait nsIGeolocationUpdateCoerce {
    fn coerce_from(v: &nsIGeolocationUpdate) -> &Self;
}

impl nsIGeolocationUpdateCoerce for nsIGeolocationUpdate {
    #[inline]
    fn coerce_from(v: &nsIGeolocationUpdate) -> &Self {
        v
    }
}

impl nsIGeolocationUpdate {
    #[inline]
    pub fn coerce<T: nsIGeolocationUpdateCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIGeolocationUpdate {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIGeolocationUpdateCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGeolocationUpdate) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIGeolocationUpdateVTable {
    pub __base: nsISupportsVTable,

    /* void update (in nsIDOMGeoPosition position); */
    pub update: unsafe extern "C" fn (this: *const nsIGeolocationUpdate, position: *const nsIDOMGeoPosition) -> nsresult,

    /* void notifyError (in unsigned short error); */
    pub notifyError: unsafe extern "C" fn (this: *const nsIGeolocationUpdate, error: libc::uint16_t) -> nsresult,

}


impl nsIGeolocationUpdate {
    /* void update (in nsIDOMGeoPosition position); */
    #[inline]
    pub unsafe fn update(&self, position: Option<&nsIDOMGeoPosition>) -> Result<(), nsresult> {

        match ((*self.vtable).update)(self as *const _, position.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void notifyError (in unsigned short error); */
    #[inline]
    pub unsafe fn notifyError(&self, error: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).notifyError)(self as *const _, error) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIGeolocationProvider {
    vtable: *const nsIGeolocationProviderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIGeolocationProvider {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xac4a133b, 0x9f92, 0x4f7c,
            [0xb3, 0x69, 0xd4, 0x0c, 0xb6, 0xb1, 0x76, 0x50])
    }
}

unsafe impl RefCounted for nsIGeolocationProvider {
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
pub trait nsIGeolocationProviderCoerce {
    fn coerce_from(v: &nsIGeolocationProvider) -> &Self;
}

impl nsIGeolocationProviderCoerce for nsIGeolocationProvider {
    #[inline]
    fn coerce_from(v: &nsIGeolocationProvider) -> &Self {
        v
    }
}

impl nsIGeolocationProvider {
    #[inline]
    pub fn coerce<T: nsIGeolocationProviderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIGeolocationProvider {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIGeolocationProviderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGeolocationProvider) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIGeolocationProviderVTable {
    pub __base: nsISupportsVTable,

    /* void startup (); */
    pub startup: unsafe extern "C" fn (this: *const nsIGeolocationProvider) -> nsresult,

    /* void watch (in nsIGeolocationUpdate callback); */
    pub watch: unsafe extern "C" fn (this: *const nsIGeolocationProvider, callback: *const nsIGeolocationUpdate) -> nsresult,

    /* void shutdown (); */
    pub shutdown: unsafe extern "C" fn (this: *const nsIGeolocationProvider) -> nsresult,

    /* void setHighAccuracy (in boolean enable); */
    pub setHighAccuracy: unsafe extern "C" fn (this: *const nsIGeolocationProvider, enable: bool) -> nsresult,

}


impl nsIGeolocationProvider {
    /* void startup (); */
    #[inline]
    pub unsafe fn startup(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).startup)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void watch (in nsIGeolocationUpdate callback); */
    #[inline]
    pub unsafe fn watch(&self, callback: Option<&nsIGeolocationUpdate>) -> Result<(), nsresult> {

        match ((*self.vtable).watch)(self as *const _, callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void shutdown (); */
    #[inline]
    pub unsafe fn shutdown(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).shutdown)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setHighAccuracy (in boolean enable); */
    #[inline]
    pub unsafe fn setHighAccuracy(&self, enable: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setHighAccuracy)(self as *const _, enable) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


