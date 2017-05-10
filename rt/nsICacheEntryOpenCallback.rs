//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICacheEntryOpenCallback.idl
//


pub mod nsICacheEntryOpenCallback_consts {
    pub const ENTRY_WANTED: i64 = 0;
    pub const RECHECK_AFTER_WRITE_FINISHED: i64 = 1;
    pub const ENTRY_NEEDS_REVALIDATION: i64 = 2;
    pub const ENTRY_NOT_WANTED: i64 = 3;
}


#[repr(C)]
pub struct nsICacheEntryOpenCallback {
    vtable: *const nsICacheEntryOpenCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICacheEntryOpenCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1fc9fe11, 0xc6ac, 0x4748,
            [0x94, 0xbd, 0x85, 0x55, 0xa5, 0xa1, 0x2b, 0x94])
    }
}

unsafe impl RefCounted for nsICacheEntryOpenCallback {
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
pub trait nsICacheEntryOpenCallbackCoerce {
    fn coerce_from(v: &nsICacheEntryOpenCallback) -> &Self;
}

impl nsICacheEntryOpenCallbackCoerce for nsICacheEntryOpenCallback {
    #[inline]
    fn coerce_from(v: &nsICacheEntryOpenCallback) -> &Self {
        v
    }
}

impl nsICacheEntryOpenCallback {
    #[inline]
    pub fn coerce<T: nsICacheEntryOpenCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICacheEntryOpenCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICacheEntryOpenCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheEntryOpenCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICacheEntryOpenCallbackVTable {
    pub __base: nsISupportsVTable,

    /* unsigned long onCacheEntryCheck (in nsICacheEntry aEntry, in nsIApplicationCache aApplicationCache); */
    pub onCacheEntryCheck: unsafe extern "C" fn (this: *const nsICacheEntryOpenCallback, aEntry: *const nsICacheEntry, aApplicationCache: *const nsIApplicationCache, _retval: *mut libc::uint32_t) -> nsresult,

    /* void onCacheEntryAvailable (in nsICacheEntry aEntry, in boolean aNew, in nsIApplicationCache aApplicationCache, in nsresult aResult); */
    pub onCacheEntryAvailable: unsafe extern "C" fn (this: *const nsICacheEntryOpenCallback, aEntry: *const nsICacheEntry, aNew: bool, aApplicationCache: *const nsIApplicationCache, aResult: nsresult) -> nsresult,

}


impl nsICacheEntryOpenCallback {
    /* unsigned long onCacheEntryCheck (in nsICacheEntry aEntry, in nsIApplicationCache aApplicationCache); */
    #[inline]
    pub unsafe fn onCacheEntryCheck(&self, aEntry: Option<&nsICacheEntry>, aApplicationCache: Option<&nsIApplicationCache>) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).onCacheEntryCheck)(self as *const _, aEntry.map_or(::std::ptr::null(), |x| x as *const _), aApplicationCache.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void onCacheEntryAvailable (in nsICacheEntry aEntry, in boolean aNew, in nsIApplicationCache aApplicationCache, in nsresult aResult); */
    #[inline]
    pub unsafe fn onCacheEntryAvailable(&self, aEntry: Option<&nsICacheEntry>, aNew: bool, aApplicationCache: Option<&nsIApplicationCache>, aResult: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).onCacheEntryAvailable)(self as *const _, aEntry.map_or(::std::ptr::null(), |x| x as *const _), aNew, aApplicationCache.map_or(::std::ptr::null(), |x| x as *const _), aResult) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


