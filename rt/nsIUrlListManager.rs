//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUrlListManager.idl
//


#[repr(C)]
pub struct nsIUrlListManagerCallback {
    vtable: *const nsIUrlListManagerCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUrlListManagerCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfa4caf12, 0xd057, 0x4e7e,
            [0x81, 0xe9, 0xce, 0x06, 0x6c, 0xee, 0xe9, 0x0b])
    }
}

unsafe impl RefCounted for nsIUrlListManagerCallback {
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
pub trait nsIUrlListManagerCallbackCoerce {
    fn coerce_from(v: &nsIUrlListManagerCallback) -> &Self;
}

impl nsIUrlListManagerCallbackCoerce for nsIUrlListManagerCallback {
    #[inline]
    fn coerce_from(v: &nsIUrlListManagerCallback) -> &Self {
        v
    }
}

impl nsIUrlListManagerCallback {
    #[inline]
    pub fn coerce<T: nsIUrlListManagerCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUrlListManagerCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUrlListManagerCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlListManagerCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUrlListManagerCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void handleEvent (in ACString value); */
    pub handleEvent: unsafe extern "C" fn (this: *const nsIUrlListManagerCallback, value: *const nsACString) -> nsresult,

}


impl nsIUrlListManagerCallback {
    /* void handleEvent (in ACString value); */
    #[inline]
    pub unsafe fn handleEvent(&self, value: &[u8]) -> Result<(), nsresult> {
        let value = nsCString::from(value);
        match ((*self.vtable).handleEvent)(self as *const _, &*value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIUrlListManager {
    vtable: *const nsIUrlListManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUrlListManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd60a08ee, 0x5c83, 0x4eb6,
            [0xbd, 0xfb, 0x79, 0xfd, 0x07, 0x16, 0x50, 0x1e])
    }
}

unsafe impl RefCounted for nsIUrlListManager {
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
pub trait nsIUrlListManagerCoerce {
    fn coerce_from(v: &nsIUrlListManager) -> &Self;
}

impl nsIUrlListManagerCoerce for nsIUrlListManager {
    #[inline]
    fn coerce_from(v: &nsIUrlListManager) -> &Self {
        v
    }
}

impl nsIUrlListManager {
    #[inline]
    pub fn coerce<T: nsIUrlListManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUrlListManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUrlListManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlListManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUrlListManagerVTable {
    pub __base: nsISupportsVTable,

    /* ACString getGethashUrl (in ACString tableName); */
    pub getGethashUrl: unsafe extern "C" fn (this: *const nsIUrlListManager, tableName: *const nsACString, _retval: *mut nsACString) -> nsresult,

    /* ACString getUpdateUrl (in ACString tableName); */
    pub getUpdateUrl: unsafe extern "C" fn (this: *const nsIUrlListManager, tableName: *const nsACString, _retval: *mut nsACString) -> nsresult,

    /* boolean registerTable (in ACString tableName, in ACString providerName, in ACString updateUrl, in ACString gethashUrl); */
    pub registerTable: unsafe extern "C" fn (this: *const nsIUrlListManager, tableName: *const nsACString, providerName: *const nsACString, updateUrl: *const nsACString, gethashUrl: *const nsACString, _retval: *mut bool) -> nsresult,

    /* void enableUpdate (in ACString tableName); */
    pub enableUpdate: unsafe extern "C" fn (this: *const nsIUrlListManager, tableName: *const nsACString) -> nsresult,

    /* void disableUpdate (in ACString tableName); */
    pub disableUpdate: unsafe extern "C" fn (this: *const nsIUrlListManager, tableName: *const nsACString) -> nsresult,

    /* void maybeToggleUpdateChecking (); */
    pub maybeToggleUpdateChecking: unsafe extern "C" fn (this: *const nsIUrlListManager) -> nsresult,

    /* void safeLookup (in nsIPrincipal key, in nsIUrlListManagerCallback cb); */
    pub safeLookup: unsafe extern "C" fn (this: *const nsIUrlListManager, key: *const nsIPrincipal, cb: *const nsIUrlListManagerCallback) -> nsresult,

    /* boolean checkForUpdates (in ACString updateUrl); */
    pub checkForUpdates: unsafe extern "C" fn (this: *const nsIUrlListManager, updateUrl: *const nsACString, _retval: *mut bool) -> nsresult,

}


impl nsIUrlListManager {
    /* ACString getGethashUrl (in ACString tableName); */
    #[inline]
    pub unsafe fn getGethashUrl(&self, tableName: &[u8]) -> Result<nsCString, nsresult> {
        let tableName = nsCString::from(tableName);
        let mut _retval = nsCString::new();
        match ((*self.vtable).getGethashUrl)(self as *const _, &*tableName, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString getUpdateUrl (in ACString tableName); */
    #[inline]
    pub unsafe fn getUpdateUrl(&self, tableName: &[u8]) -> Result<nsCString, nsresult> {
        let tableName = nsCString::from(tableName);
        let mut _retval = nsCString::new();
        match ((*self.vtable).getUpdateUrl)(self as *const _, &*tableName, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean registerTable (in ACString tableName, in ACString providerName, in ACString updateUrl, in ACString gethashUrl); */
    #[inline]
    pub unsafe fn registerTable(&self, tableName: &[u8], providerName: &[u8], updateUrl: &[u8], gethashUrl: &[u8]) -> Result<bool, nsresult> {
        let tableName = nsCString::from(tableName);
        let providerName = nsCString::from(providerName);
        let updateUrl = nsCString::from(updateUrl);
        let gethashUrl = nsCString::from(gethashUrl);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).registerTable)(self as *const _, &*tableName, &*providerName, &*updateUrl, &*gethashUrl, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void enableUpdate (in ACString tableName); */
    #[inline]
    pub unsafe fn enableUpdate(&self, tableName: &[u8]) -> Result<(), nsresult> {
        let tableName = nsCString::from(tableName);
        match ((*self.vtable).enableUpdate)(self as *const _, &*tableName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void disableUpdate (in ACString tableName); */
    #[inline]
    pub unsafe fn disableUpdate(&self, tableName: &[u8]) -> Result<(), nsresult> {
        let tableName = nsCString::from(tableName);
        match ((*self.vtable).disableUpdate)(self as *const _, &*tableName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void maybeToggleUpdateChecking (); */
    #[inline]
    pub unsafe fn maybeToggleUpdateChecking(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).maybeToggleUpdateChecking)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void safeLookup (in nsIPrincipal key, in nsIUrlListManagerCallback cb); */
    #[inline]
    pub unsafe fn safeLookup(&self, key: Option<&nsIPrincipal>, cb: Option<&nsIUrlListManagerCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).safeLookup)(self as *const _, key.map_or(::std::ptr::null(), |x| x as *const _), cb.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean checkForUpdates (in ACString updateUrl); */
    #[inline]
    pub unsafe fn checkForUpdates(&self, updateUrl: &[u8]) -> Result<bool, nsresult> {
        let updateUrl = nsCString::from(updateUrl);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).checkForUpdates)(self as *const _, &*updateUrl, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


