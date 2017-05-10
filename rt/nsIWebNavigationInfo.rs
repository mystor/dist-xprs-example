//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebNavigationInfo.idl
//


pub mod nsIWebNavigationInfo_consts {
    pub const UNSUPPORTED: i64 = 0;
    pub const IMAGE: i64 = 1;
    pub const PLUGIN: i64 = 2;
    pub const OTHER: i64 = 32768;
}


#[repr(C)]
pub struct nsIWebNavigationInfo {
    vtable: *const nsIWebNavigationInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebNavigationInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x62a93afb, 0x93a1, 0x465c,
            [0x84, 0xc8, 0x04, 0x32, 0x26, 0x42, 0x29, 0xde])
    }
}

unsafe impl RefCounted for nsIWebNavigationInfo {
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
pub trait nsIWebNavigationInfoCoerce {
    fn coerce_from(v: &nsIWebNavigationInfo) -> &Self;
}

impl nsIWebNavigationInfoCoerce for nsIWebNavigationInfo {
    #[inline]
    fn coerce_from(v: &nsIWebNavigationInfo) -> &Self {
        v
    }
}

impl nsIWebNavigationInfo {
    #[inline]
    pub fn coerce<T: nsIWebNavigationInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebNavigationInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebNavigationInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebNavigationInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebNavigationInfoVTable {
    pub __base: nsISupportsVTable,

    /* unsigned long isTypeSupported (in ACString aType, in nsIWebNavigation aWebNav); */
    pub isTypeSupported: unsafe extern "C" fn (this: *const nsIWebNavigationInfo, aType: *const nsACString, aWebNav: *const nsIWebNavigation, _retval: *mut libc::uint32_t) -> nsresult,

}


impl nsIWebNavigationInfo {
    /* unsigned long isTypeSupported (in ACString aType, in nsIWebNavigation aWebNav); */
    #[inline]
    pub unsafe fn isTypeSupported(&self, aType: &[u8], aWebNav: Option<&nsIWebNavigation>) -> Result<libc::uint32_t, nsresult> {
        let aType = nsCString::from(aType);
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).isTypeSupported)(self as *const _, &*aType, aWebNav.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


