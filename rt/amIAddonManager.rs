//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/amIAddonManager.idl
//


#[repr(C)]
pub struct amIAddonManager {
    vtable: *const amIAddonManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for amIAddonManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7b45d82d, 0x7ad5, 0x48d7,
            [0x9b, 0x05, 0xf3, 0x2e, 0xb9, 0x81, 0x8c, 0xd4])
    }
}

unsafe impl RefCounted for amIAddonManager {
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
pub trait amIAddonManagerCoerce {
    fn coerce_from(v: &amIAddonManager) -> &Self;
}

impl amIAddonManagerCoerce for amIAddonManager {
    #[inline]
    fn coerce_from(v: &amIAddonManager) -> &Self {
        v
    }
}

impl amIAddonManager {
    #[inline]
    pub fn coerce<T: amIAddonManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for amIAddonManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> amIAddonManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &amIAddonManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct amIAddonManagerVTable {
    pub __base: nsISupportsVTable,

    /* boolean mapURIToAddonID (in nsIURI aURI, out AUTF8String aID); */
    pub mapURIToAddonID: unsafe extern "C" fn (this: *const amIAddonManager, aURI: *const nsIURI, aID: *mut nsACString, _retval: *mut bool) -> nsresult,

}


impl amIAddonManager {
    /* boolean mapURIToAddonID (in nsIURI aURI, out AUTF8String aID); */
    #[inline]
    pub unsafe fn mapURIToAddonID(&self, aURI: Option<&nsIURI>) -> Result<(nsCString, bool), nsresult> {
        let mut aID = nsCString::new();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).mapURIToAddonID)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &mut *aID, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aID, _retval))
    }

}


