//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUrlClassifierDBService.idl
//


#[repr(C)]
pub struct nsIUrlClassifierCallback {
    vtable: *const nsIUrlClassifierCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUrlClassifierCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4ca27b6b, 0xa674, 0x4b3d,
            [0xab, 0x30, 0xd2, 0x1e, 0x2d, 0xa2, 0xdf, 0xfb])
    }
}

unsafe impl RefCounted for nsIUrlClassifierCallback {
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
pub trait nsIUrlClassifierCallbackCoerce {
    fn coerce_from(v: &nsIUrlClassifierCallback) -> &Self;
}

impl nsIUrlClassifierCallbackCoerce for nsIUrlClassifierCallback {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierCallback) -> &Self {
        v
    }
}

impl nsIUrlClassifierCallback {
    #[inline]
    pub fn coerce<T: nsIUrlClassifierCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUrlClassifierCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUrlClassifierCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUrlClassifierCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void handleEvent (in ACString value); */
    pub handleEvent: unsafe extern "C" fn (this: *const nsIUrlClassifierCallback, value: *const nsACString) -> nsresult,

}


impl nsIUrlClassifierCallback {
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
pub struct nsIUrlClassifierUpdateObserver {
    vtable: *const nsIUrlClassifierUpdateObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUrlClassifierUpdateObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9fa11561, 0x5816, 0x4e1b,
            [0xbc, 0xc9, 0xb6, 0x29, 0xca, 0x05, 0xcc, 0xe6])
    }
}

unsafe impl RefCounted for nsIUrlClassifierUpdateObserver {
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
pub trait nsIUrlClassifierUpdateObserverCoerce {
    fn coerce_from(v: &nsIUrlClassifierUpdateObserver) -> &Self;
}

impl nsIUrlClassifierUpdateObserverCoerce for nsIUrlClassifierUpdateObserver {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierUpdateObserver) -> &Self {
        v
    }
}

impl nsIUrlClassifierUpdateObserver {
    #[inline]
    pub fn coerce<T: nsIUrlClassifierUpdateObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUrlClassifierUpdateObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUrlClassifierUpdateObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierUpdateObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUrlClassifierUpdateObserverVTable {
    pub __base: nsISupportsVTable,

    /* void updateUrlRequested (in ACString url, in ACString table); */
    pub updateUrlRequested: unsafe extern "C" fn (this: *const nsIUrlClassifierUpdateObserver, url: *const nsACString, table: *const nsACString) -> nsresult,

    /* void streamFinished (in nsresult status, in unsigned long delay); */
    pub streamFinished: unsafe extern "C" fn (this: *const nsIUrlClassifierUpdateObserver, status: nsresult, delay: libc::uint32_t) -> nsresult,

    /* void updateError (in nsresult error); */
    pub updateError: unsafe extern "C" fn (this: *const nsIUrlClassifierUpdateObserver, error: nsresult) -> nsresult,

    /* void updateSuccess (in unsigned long requestedTimeout); */
    pub updateSuccess: unsafe extern "C" fn (this: *const nsIUrlClassifierUpdateObserver, requestedTimeout: libc::uint32_t) -> nsresult,

}


impl nsIUrlClassifierUpdateObserver {
    /* void updateUrlRequested (in ACString url, in ACString table); */
    #[inline]
    pub unsafe fn updateUrlRequested(&self, url: &[u8], table: &[u8]) -> Result<(), nsresult> {
        let url = nsCString::from(url);
        let table = nsCString::from(table);
        match ((*self.vtable).updateUrlRequested)(self as *const _, &*url, &*table) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void streamFinished (in nsresult status, in unsigned long delay); */
    #[inline]
    pub unsafe fn streamFinished(&self, status: nsresult, delay: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).streamFinished)(self as *const _, status, delay) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void updateError (in nsresult error); */
    #[inline]
    pub unsafe fn updateError(&self, error: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).updateError)(self as *const _, error) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void updateSuccess (in unsigned long requestedTimeout); */
    #[inline]
    pub unsafe fn updateSuccess(&self, requestedTimeout: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).updateSuccess)(self as *const _, requestedTimeout) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIUrlClassifierDBService {
    vtable: *const nsIUrlClassifierDBServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUrlClassifierDBService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7a258022, 0x6765, 0x11e5,
            [0xb3, 0x79, 0xb3, 0x7b, 0x1f, 0x23, 0x54, 0xbe])
    }
}

unsafe impl RefCounted for nsIUrlClassifierDBService {
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
pub trait nsIUrlClassifierDBServiceCoerce {
    fn coerce_from(v: &nsIUrlClassifierDBService) -> &Self;
}

impl nsIUrlClassifierDBServiceCoerce for nsIUrlClassifierDBService {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierDBService) -> &Self {
        v
    }
}

impl nsIUrlClassifierDBService {
    #[inline]
    pub fn coerce<T: nsIUrlClassifierDBServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUrlClassifierDBService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUrlClassifierDBServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierDBService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUrlClassifierDBServiceVTable {
    pub __base: nsISupportsVTable,

    /* void lookup (in nsIPrincipal principal, in ACString tables, in nsIUrlClassifierCallback c); */
    pub lookup: unsafe extern "C" fn (this: *const nsIUrlClassifierDBService, principal: *const nsIPrincipal, tables: *const nsACString, c: *const nsIUrlClassifierCallback) -> nsresult,

    /* void getTables (in nsIUrlClassifierCallback c); */
    pub getTables: unsafe extern "C" fn (this: *const nsIUrlClassifierDBService, c: *const nsIUrlClassifierCallback) -> nsresult,

    /* void setHashCompleter (in ACString tableName, in nsIUrlClassifierHashCompleter completer); */
    pub setHashCompleter: unsafe extern "C" fn (this: *const nsIUrlClassifierDBService, tableName: *const nsACString, completer: *const nsIUrlClassifierHashCompleter) -> nsresult,

    /* void setLastUpdateTime (in ACString tableName, in unsigned long long lastUpdateTime); */
    pub setLastUpdateTime: unsafe extern "C" fn (this: *const nsIUrlClassifierDBService, tableName: *const nsACString, lastUpdateTime: libc::uint64_t) -> nsresult,

    /* void clearLastResults (); */
    pub clearLastResults: unsafe extern "C" fn (this: *const nsIUrlClassifierDBService) -> nsresult,

    /* void beginUpdate (in nsIUrlClassifierUpdateObserver updater, in ACString tables); */
    pub beginUpdate: unsafe extern "C" fn (this: *const nsIUrlClassifierDBService, updater: *const nsIUrlClassifierUpdateObserver, tables: *const nsACString) -> nsresult,

    /* void beginStream (in ACString table); */
    pub beginStream: unsafe extern "C" fn (this: *const nsIUrlClassifierDBService, table: *const nsACString) -> nsresult,

    /* void updateStream (in ACString updateChunk); */
    pub updateStream: unsafe extern "C" fn (this: *const nsIUrlClassifierDBService, updateChunk: *const nsACString) -> nsresult,

    /* void finishStream (); */
    pub finishStream: unsafe extern "C" fn (this: *const nsIUrlClassifierDBService) -> nsresult,

    /* void finishUpdate (); */
    pub finishUpdate: unsafe extern "C" fn (this: *const nsIUrlClassifierDBService) -> nsresult,

    /* void cancelUpdate (); */
    pub cancelUpdate: unsafe extern "C" fn (this: *const nsIUrlClassifierDBService) -> nsresult,

    /* void resetDatabase (); */
    pub resetDatabase: unsafe extern "C" fn (this: *const nsIUrlClassifierDBService) -> nsresult,

    /* void reloadDatabase (); */
    pub reloadDatabase: unsafe extern "C" fn (this: *const nsIUrlClassifierDBService) -> nsresult,

}


impl nsIUrlClassifierDBService {
    /* void lookup (in nsIPrincipal principal, in ACString tables, in nsIUrlClassifierCallback c); */
    #[inline]
    pub unsafe fn lookup(&self, principal: Option<&nsIPrincipal>, tables: &[u8], c: Option<&nsIUrlClassifierCallback>) -> Result<(), nsresult> {
        let tables = nsCString::from(tables);
        match ((*self.vtable).lookup)(self as *const _, principal.map_or(::std::ptr::null(), |x| x as *const _), &*tables, c.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getTables (in nsIUrlClassifierCallback c); */
    #[inline]
    pub unsafe fn getTables(&self, c: Option<&nsIUrlClassifierCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).getTables)(self as *const _, c.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setHashCompleter (in ACString tableName, in nsIUrlClassifierHashCompleter completer); */
    #[inline]
    pub unsafe fn setHashCompleter(&self, tableName: &[u8], completer: Option<&nsIUrlClassifierHashCompleter>) -> Result<(), nsresult> {
        let tableName = nsCString::from(tableName);
        match ((*self.vtable).setHashCompleter)(self as *const _, &*tableName, completer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setLastUpdateTime (in ACString tableName, in unsigned long long lastUpdateTime); */
    #[inline]
    pub unsafe fn setLastUpdateTime(&self, tableName: &[u8], lastUpdateTime: libc::uint64_t) -> Result<(), nsresult> {
        let tableName = nsCString::from(tableName);
        match ((*self.vtable).setLastUpdateTime)(self as *const _, &*tableName, lastUpdateTime) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void clearLastResults (); */
    #[inline]
    pub unsafe fn clearLastResults(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).clearLastResults)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void beginUpdate (in nsIUrlClassifierUpdateObserver updater, in ACString tables); */
    #[inline]
    pub unsafe fn beginUpdate(&self, updater: Option<&nsIUrlClassifierUpdateObserver>, tables: &[u8]) -> Result<(), nsresult> {
        let tables = nsCString::from(tables);
        match ((*self.vtable).beginUpdate)(self as *const _, updater.map_or(::std::ptr::null(), |x| x as *const _), &*tables) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void beginStream (in ACString table); */
    #[inline]
    pub unsafe fn beginStream(&self, table: &[u8]) -> Result<(), nsresult> {
        let table = nsCString::from(table);
        match ((*self.vtable).beginStream)(self as *const _, &*table) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void updateStream (in ACString updateChunk); */
    #[inline]
    pub unsafe fn updateStream(&self, updateChunk: &[u8]) -> Result<(), nsresult> {
        let updateChunk = nsCString::from(updateChunk);
        match ((*self.vtable).updateStream)(self as *const _, &*updateChunk) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void finishStream (); */
    #[inline]
    pub unsafe fn finishStream(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).finishStream)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void finishUpdate (); */
    #[inline]
    pub unsafe fn finishUpdate(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).finishUpdate)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void cancelUpdate (); */
    #[inline]
    pub unsafe fn cancelUpdate(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).cancelUpdate)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void resetDatabase (); */
    #[inline]
    pub unsafe fn resetDatabase(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).resetDatabase)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void reloadDatabase (); */
    #[inline]
    pub unsafe fn reloadDatabase(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).reloadDatabase)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIUrlClassifierLookupCallback {
    vtable: *const nsIUrlClassifierLookupCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUrlClassifierLookupCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb903dc8f, 0xdff1, 0x42fe,
            [0x89, 0x4b, 0x36, 0xe7, 0xa5, 0x9b, 0xb8, 0x01])
    }
}

unsafe impl RefCounted for nsIUrlClassifierLookupCallback {
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
pub trait nsIUrlClassifierLookupCallbackCoerce {
    fn coerce_from(v: &nsIUrlClassifierLookupCallback) -> &Self;
}

impl nsIUrlClassifierLookupCallbackCoerce for nsIUrlClassifierLookupCallback {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierLookupCallback) -> &Self {
        v
    }
}

impl nsIUrlClassifierLookupCallback {
    #[inline]
    pub fn coerce<T: nsIUrlClassifierLookupCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUrlClassifierLookupCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUrlClassifierLookupCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierLookupCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUrlClassifierLookupCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void lookupComplete (in ResultArray results); */
    /// Unable to call function as its signature contains a non-rust type
    pub lookupComplete: *const ::libc::c_void,

}


impl nsIUrlClassifierLookupCallback {
    /* void lookupComplete (in ResultArray results); */


}


#[repr(C)]
pub struct nsIUrlClassifierClassifyCallback {
    vtable: *const nsIUrlClassifierClassifyCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUrlClassifierClassifyCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x091adf98, 0x28a5, 0x473d,
            [0x8d, 0xec, 0x5b, 0x34, 0xb4, 0xe6, 0x24, 0x96])
    }
}

unsafe impl RefCounted for nsIUrlClassifierClassifyCallback {
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
pub trait nsIUrlClassifierClassifyCallbackCoerce {
    fn coerce_from(v: &nsIUrlClassifierClassifyCallback) -> &Self;
}

impl nsIUrlClassifierClassifyCallbackCoerce for nsIUrlClassifierClassifyCallback {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierClassifyCallback) -> &Self {
        v
    }
}

impl nsIUrlClassifierClassifyCallback {
    #[inline]
    pub fn coerce<T: nsIUrlClassifierClassifyCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUrlClassifierClassifyCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUrlClassifierClassifyCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierClassifyCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUrlClassifierClassifyCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void handleResult (in ACString aList, in ACString aPrefix); */
    pub handleResult: unsafe extern "C" fn (this: *const nsIUrlClassifierClassifyCallback, aList: *const nsACString, aPrefix: *const nsACString) -> nsresult,

}


impl nsIUrlClassifierClassifyCallback {
    /* void handleResult (in ACString aList, in ACString aPrefix); */
    #[inline]
    pub unsafe fn handleResult(&self, aList: &[u8], aPrefix: &[u8]) -> Result<(), nsresult> {
        let aList = nsCString::from(aList);
        let aPrefix = nsCString::from(aPrefix);
        match ((*self.vtable).handleResult)(self as *const _, &*aList, &*aPrefix) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


