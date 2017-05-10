//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBFCacheEntry.idl
//


#[repr(C)]
pub struct nsIBFCacheEntry {
    vtable: *const nsIBFCacheEntryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIBFCacheEntry {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa576060e, 0xc7df, 0x4d81,
            [0xaa, 0x8c, 0x5b, 0x52, 0xbd, 0x6a, 0xd2, 0x5d])
    }
}

unsafe impl RefCounted for nsIBFCacheEntry {
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
pub trait nsIBFCacheEntryCoerce {
    fn coerce_from(v: &nsIBFCacheEntry) -> &Self;
}

impl nsIBFCacheEntryCoerce for nsIBFCacheEntry {
    #[inline]
    fn coerce_from(v: &nsIBFCacheEntry) -> &Self {
        v
    }
}

impl nsIBFCacheEntry {
    #[inline]
    pub fn coerce<T: nsIBFCacheEntryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIBFCacheEntry {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIBFCacheEntryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBFCacheEntry) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIBFCacheEntryVTable {
    pub __base: nsISupportsVTable,

    /* void RemoveFromBFCacheSync (); */
    pub RemoveFromBFCacheSync: unsafe extern "C" fn (this: *const nsIBFCacheEntry) -> nsresult,

    /* void RemoveFromBFCacheAsync (); */
    pub RemoveFromBFCacheAsync: unsafe extern "C" fn (this: *const nsIBFCacheEntry) -> nsresult,

    /* readonly attribute unsigned long long ID; */
    pub get_ID: unsafe extern "C" fn (this: *const nsIBFCacheEntry, aID: *mut libc::uint64_t) -> nsresult,

}


impl nsIBFCacheEntry {
    /* void RemoveFromBFCacheSync (); */
    #[inline]
    pub unsafe fn RemoveFromBFCacheSync(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).RemoveFromBFCacheSync)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void RemoveFromBFCacheAsync (); */
    #[inline]
    pub unsafe fn RemoveFromBFCacheAsync(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).RemoveFromBFCacheAsync)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute unsigned long long ID; */
    #[inline]
    pub unsafe fn get_ID(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_ID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


