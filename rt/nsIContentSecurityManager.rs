//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentSecurityManager.idl
//


#[repr(C)]
pub struct nsIContentSecurityManager {
    vtable: *const nsIContentSecurityManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentSecurityManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3a9a1818, 0x2ae8, 0x4ec5,
            [0xa3, 0x40, 0x8b, 0x29, 0xd3, 0x1f, 0xca, 0x3b])
    }
}

unsafe impl RefCounted for nsIContentSecurityManager {
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
pub trait nsIContentSecurityManagerCoerce {
    fn coerce_from(v: &nsIContentSecurityManager) -> &Self;
}

impl nsIContentSecurityManagerCoerce for nsIContentSecurityManager {
    #[inline]
    fn coerce_from(v: &nsIContentSecurityManager) -> &Self {
        v
    }
}

impl nsIContentSecurityManager {
    #[inline]
    pub fn coerce<T: nsIContentSecurityManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentSecurityManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContentSecurityManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentSecurityManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentSecurityManagerVTable {
    pub __base: nsISupportsVTable,

    /* nsIStreamListener performSecurityCheck (in nsIChannel aChannel, in nsIStreamListener aStreamListener); */
    pub performSecurityCheck: unsafe extern "C" fn (this: *const nsIContentSecurityManager, aChannel: *const nsIChannel, aStreamListener: *const nsIStreamListener, _retval: *mut *const nsIStreamListener) -> nsresult,

    /* boolean isOriginPotentiallyTrustworthy (in nsIPrincipal aPrincipal); */
    pub isOriginPotentiallyTrustworthy: unsafe extern "C" fn (this: *const nsIContentSecurityManager, aPrincipal: *const nsIPrincipal, _retval: *mut bool) -> nsresult,

}


impl nsIContentSecurityManager {
    /* nsIStreamListener performSecurityCheck (in nsIChannel aChannel, in nsIStreamListener aStreamListener); */
    #[inline]
    pub unsafe fn performSecurityCheck(&self, aChannel: Option<&nsIChannel>, aStreamListener: Option<&nsIStreamListener>) -> Result<Option<RefPtr<nsIStreamListener>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).performSecurityCheck)(self as *const _, aChannel.map_or(::std::ptr::null(), |x| x as *const _), aStreamListener.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean isOriginPotentiallyTrustworthy (in nsIPrincipal aPrincipal); */
    #[inline]
    pub unsafe fn isOriginPotentiallyTrustworthy(&self, aPrincipal: Option<&nsIPrincipal>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isOriginPotentiallyTrustworthy)(self as *const _, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


