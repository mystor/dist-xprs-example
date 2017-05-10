//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUrlClassifierHashCompleter.idl
//


#[repr(C)]
pub struct nsIFullHashMatch {
    vtable: *const nsIFullHashMatchVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFullHashMatch {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xaabeb50e, 0xd9f7, 0x418e,
            [0x94, 0x69, 0x2c, 0xd9, 0x60, 0x89, 0x58, 0xc0])
    }
}

unsafe impl RefCounted for nsIFullHashMatch {
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
pub trait nsIFullHashMatchCoerce {
    fn coerce_from(v: &nsIFullHashMatch) -> &Self;
}

impl nsIFullHashMatchCoerce for nsIFullHashMatch {
    #[inline]
    fn coerce_from(v: &nsIFullHashMatch) -> &Self {
        v
    }
}

impl nsIFullHashMatch {
    #[inline]
    pub fn coerce<T: nsIFullHashMatchCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFullHashMatch {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFullHashMatchCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFullHashMatch) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFullHashMatchVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString tableName; */
    pub get_tableName: unsafe extern "C" fn (this: *const nsIFullHashMatch, aTableName: *mut nsACString) -> nsresult,

    /* readonly attribute ACString fullHash; */
    pub get_fullHash: unsafe extern "C" fn (this: *const nsIFullHashMatch, aFullHash: *mut nsACString) -> nsresult,

    /* readonly attribute uint32_t cacheDuration; */
    pub get_cacheDuration: unsafe extern "C" fn (this: *const nsIFullHashMatch, aCacheDuration: *mut uint32_t) -> nsresult,

}


impl nsIFullHashMatch {
    /* readonly attribute ACString tableName; */
    #[inline]
    pub unsafe fn get_tableName(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_tableName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString fullHash; */
    #[inline]
    pub unsafe fn get_fullHash(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_fullHash)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute uint32_t cacheDuration; */
    #[inline]
    pub unsafe fn get_cacheDuration(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_cacheDuration)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIUrlClassifierHashCompleterCallback {
    vtable: *const nsIUrlClassifierHashCompleterCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUrlClassifierHashCompleterCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xda16de40, 0xdf26, 0x414d,
            [0xbd, 0xe7, 0xc4, 0xfa, 0xf4, 0x50, 0x48, 0x68])
    }
}

unsafe impl RefCounted for nsIUrlClassifierHashCompleterCallback {
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
pub trait nsIUrlClassifierHashCompleterCallbackCoerce {
    fn coerce_from(v: &nsIUrlClassifierHashCompleterCallback) -> &Self;
}

impl nsIUrlClassifierHashCompleterCallbackCoerce for nsIUrlClassifierHashCompleterCallback {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierHashCompleterCallback) -> &Self {
        v
    }
}

impl nsIUrlClassifierHashCompleterCallback {
    #[inline]
    pub fn coerce<T: nsIUrlClassifierHashCompleterCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUrlClassifierHashCompleterCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUrlClassifierHashCompleterCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierHashCompleterCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUrlClassifierHashCompleterCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void completionV2 (in ACString hash, in ACString table, in uint32_t chunkId); */
    pub completionV2: unsafe extern "C" fn (this: *const nsIUrlClassifierHashCompleterCallback, hash: *const nsACString, table: *const nsACString, chunkId: uint32_t) -> nsresult,

    /* void completionV4 (in ACString partialHash, in ACString table, in uint32_t negativeCacheDuration, in nsIArray fullHashes); */
    pub completionV4: unsafe extern "C" fn (this: *const nsIUrlClassifierHashCompleterCallback, partialHash: *const nsACString, table: *const nsACString, negativeCacheDuration: uint32_t, fullHashes: *const nsIArray) -> nsresult,

    /* void completionFinished (in nsresult status); */
    pub completionFinished: unsafe extern "C" fn (this: *const nsIUrlClassifierHashCompleterCallback, status: nsresult) -> nsresult,

}


impl nsIUrlClassifierHashCompleterCallback {
    /* void completionV2 (in ACString hash, in ACString table, in uint32_t chunkId); */
    #[inline]
    pub unsafe fn completionV2(&self, hash: &[u8], table: &[u8], chunkId: uint32_t) -> Result<(), nsresult> {
        let hash = nsCString::from(hash);
        let table = nsCString::from(table);
        match ((*self.vtable).completionV2)(self as *const _, &*hash, &*table, chunkId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void completionV4 (in ACString partialHash, in ACString table, in uint32_t negativeCacheDuration, in nsIArray fullHashes); */
    #[inline]
    pub unsafe fn completionV4(&self, partialHash: &[u8], table: &[u8], negativeCacheDuration: uint32_t, fullHashes: Option<&nsIArray>) -> Result<(), nsresult> {
        let partialHash = nsCString::from(partialHash);
        let table = nsCString::from(table);
        match ((*self.vtable).completionV4)(self as *const _, &*partialHash, &*table, negativeCacheDuration, fullHashes.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void completionFinished (in nsresult status); */
    #[inline]
    pub unsafe fn completionFinished(&self, status: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).completionFinished)(self as *const _, status) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIUrlClassifierHashCompleter {
    vtable: *const nsIUrlClassifierHashCompleterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUrlClassifierHashCompleter {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x231fb2ad, 0xea8a, 0x4e63,
            [0xa3, 0x31, 0xea, 0xfc, 0x3b, 0x43, 0x48, 0x11])
    }
}

unsafe impl RefCounted for nsIUrlClassifierHashCompleter {
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
pub trait nsIUrlClassifierHashCompleterCoerce {
    fn coerce_from(v: &nsIUrlClassifierHashCompleter) -> &Self;
}

impl nsIUrlClassifierHashCompleterCoerce for nsIUrlClassifierHashCompleter {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierHashCompleter) -> &Self {
        v
    }
}

impl nsIUrlClassifierHashCompleter {
    #[inline]
    pub fn coerce<T: nsIUrlClassifierHashCompleterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUrlClassifierHashCompleter {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUrlClassifierHashCompleterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierHashCompleter) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUrlClassifierHashCompleterVTable {
    pub __base: nsISupportsVTable,

    /* void complete (in ACString partialHash, in ACString gethashUrl, in ACString tableName, in nsIUrlClassifierHashCompleterCallback callback); */
    pub complete: unsafe extern "C" fn (this: *const nsIUrlClassifierHashCompleter, partialHash: *const nsACString, gethashUrl: *const nsACString, tableName: *const nsACString, callback: *const nsIUrlClassifierHashCompleterCallback) -> nsresult,

}


impl nsIUrlClassifierHashCompleter {
    /* void complete (in ACString partialHash, in ACString gethashUrl, in ACString tableName, in nsIUrlClassifierHashCompleterCallback callback); */
    #[inline]
    pub unsafe fn complete(&self, partialHash: &[u8], gethashUrl: &[u8], tableName: &[u8], callback: Option<&nsIUrlClassifierHashCompleterCallback>) -> Result<(), nsresult> {
        let partialHash = nsCString::from(partialHash);
        let gethashUrl = nsCString::from(gethashUrl);
        let tableName = nsCString::from(tableName);
        match ((*self.vtable).complete)(self as *const _, &*partialHash, &*gethashUrl, &*tableName, callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


