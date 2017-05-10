//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStartupCache.idl
//


#[repr(C)]
pub struct nsIStartupCache {
    vtable: *const nsIStartupCacheVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIStartupCache {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x25957820, 0x90a1, 0x428c,
            [0x87, 0x39, 0xb0, 0x84, 0x5d, 0x3c, 0xc5, 0x34])
    }
}

unsafe impl RefCounted for nsIStartupCache {
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
pub trait nsIStartupCacheCoerce {
    fn coerce_from(v: &nsIStartupCache) -> &Self;
}

impl nsIStartupCacheCoerce for nsIStartupCache {
    #[inline]
    fn coerce_from(v: &nsIStartupCache) -> &Self {
        v
    }
}

impl nsIStartupCache {
    #[inline]
    pub fn coerce<T: nsIStartupCacheCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIStartupCache {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIStartupCacheCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStartupCache) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIStartupCacheVTable {
    pub __base: nsISupportsVTable,

    /* uint32_t getBuffer (in string aID, out charPtr aBuffer); */
    pub getBuffer: unsafe extern "C" fn (this: *const nsIStartupCache, aID: *const libc::c_char, aBuffer: *mut *const u8, _retval: *mut uint32_t) -> nsresult,

    /* void putBuffer (in string aID, in string aBuffer, in uint32_t aLength); */
    pub putBuffer: unsafe extern "C" fn (this: *const nsIStartupCache, aID: *const libc::c_char, aBuffer: *const libc::c_char, aLength: uint32_t) -> nsresult,

    /* void invalidateCache (); */
    pub invalidateCache: unsafe extern "C" fn (this: *const nsIStartupCache) -> nsresult,

    /* nsIObjectOutputStream getDebugObjectOutputStream (in nsIObjectOutputStream aStream); */
    pub getDebugObjectOutputStream: unsafe extern "C" fn (this: *const nsIStartupCache, aStream: *const nsIObjectOutputStream, _retval: *mut *const nsIObjectOutputStream) -> nsresult,

    /* readonly attribute nsIObserver observer; */
    pub get_observer: unsafe extern "C" fn (this: *const nsIStartupCache, aObserver: *mut *const nsIObserver) -> nsresult,

}


impl nsIStartupCache {
    /* uint32_t getBuffer (in string aID, out charPtr aBuffer); */
    #[inline]
    pub unsafe fn getBuffer(&self, aID: *const libc::c_char) -> Result<(*const u8, uint32_t), nsresult> {
        let mut aBuffer: *const u8 = ::std::ptr::null();
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getBuffer)(self as *const _, aID, &mut aBuffer as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aBuffer, _retval))
    }

    /* void putBuffer (in string aID, in string aBuffer, in uint32_t aLength); */
    #[inline]
    pub unsafe fn putBuffer(&self, aID: *const libc::c_char, aBuffer: *const libc::c_char, aLength: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).putBuffer)(self as *const _, aID, aBuffer, aLength) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void invalidateCache (); */
    #[inline]
    pub unsafe fn invalidateCache(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).invalidateCache)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIObjectOutputStream getDebugObjectOutputStream (in nsIObjectOutputStream aStream); */
    #[inline]
    pub unsafe fn getDebugObjectOutputStream(&self, aStream: Option<&nsIObjectOutputStream>) -> Result<Option<RefPtr<nsIObjectOutputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getDebugObjectOutputStream)(self as *const _, aStream.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIObserver observer; */
    #[inline]
    pub unsafe fn get_observer(&self, ) -> Result<Option<RefPtr<nsIObserver>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_observer)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


