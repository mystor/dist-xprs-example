//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrivateBrowsingTrackingProtectionWhitelist.idl
//


#[repr(C)]
pub struct nsIPrivateBrowsingTrackingProtectionWhitelist {
    vtable: *const nsIPrivateBrowsingTrackingProtectionWhitelistVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPrivateBrowsingTrackingProtectionWhitelist {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc77ddfac, 0x6cd6, 0x43a9,
            [0x84, 0xe8, 0x91, 0x68, 0x2a, 0x1a, 0x7b, 0x18])
    }
}

unsafe impl RefCounted for nsIPrivateBrowsingTrackingProtectionWhitelist {
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
pub trait nsIPrivateBrowsingTrackingProtectionWhitelistCoerce {
    fn coerce_from(v: &nsIPrivateBrowsingTrackingProtectionWhitelist) -> &Self;
}

impl nsIPrivateBrowsingTrackingProtectionWhitelistCoerce for nsIPrivateBrowsingTrackingProtectionWhitelist {
    #[inline]
    fn coerce_from(v: &nsIPrivateBrowsingTrackingProtectionWhitelist) -> &Self {
        v
    }
}

impl nsIPrivateBrowsingTrackingProtectionWhitelist {
    #[inline]
    pub fn coerce<T: nsIPrivateBrowsingTrackingProtectionWhitelistCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPrivateBrowsingTrackingProtectionWhitelist {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPrivateBrowsingTrackingProtectionWhitelistCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrivateBrowsingTrackingProtectionWhitelist) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPrivateBrowsingTrackingProtectionWhitelistVTable {
    pub __base: nsISupportsVTable,

    /* void addToAllowList (in nsIURI uri); */
    pub addToAllowList: unsafe extern "C" fn (this: *const nsIPrivateBrowsingTrackingProtectionWhitelist, uri: *const nsIURI) -> nsresult,

    /* void removeFromAllowList (in nsIURI uri); */
    pub removeFromAllowList: unsafe extern "C" fn (this: *const nsIPrivateBrowsingTrackingProtectionWhitelist, uri: *const nsIURI) -> nsresult,

    /* bool existsInAllowList (in nsIURI uri); */
    pub existsInAllowList: unsafe extern "C" fn (this: *const nsIPrivateBrowsingTrackingProtectionWhitelist, uri: *const nsIURI, _retval: *mut bool) -> nsresult,

}


impl nsIPrivateBrowsingTrackingProtectionWhitelist {
    /* void addToAllowList (in nsIURI uri); */
    #[inline]
    pub unsafe fn addToAllowList(&self, uri: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).addToAllowList)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeFromAllowList (in nsIURI uri); */
    #[inline]
    pub unsafe fn removeFromAllowList(&self, uri: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).removeFromAllowList)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* bool existsInAllowList (in nsIURI uri); */
    #[inline]
    pub unsafe fn existsInAllowList(&self, uri: Option<&nsIURI>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).existsInAllowList)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


